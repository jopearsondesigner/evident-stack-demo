import { html, css, LitElement, svg } from 'lit';
import { ifDefined } from 'lit/directives/if-defined.js';

// Anchors
export const TOP = 'top';
export const LEFT = 'left';
export const BOTTOM = 'bottom';
export const RIGHT = 'right';
const ANCHORS = [TOP, LEFT, BOTTOM, RIGHT];

// Default Styling
const MARKER_SIZE = 10;
const CURVE_SHAPE_FACTOR = 0.1;
const COLOR = 'black';
const STROKE_WIDTH = 1;

// From/To observer
const ATTRIBUTES_TO_COMPARE = ['x', 'y', 'width', 'height'];
const DEFAULT_REFRESH_TIME = 16; // ms

const compareRects = (current, next) => {
  if (!current) return false;

  return !ATTRIBUTES_TO_COMPARE.some(attr => current[attr] !== next[attr]);
};

const buildPath = ({ to, from, fromBezPoint, toBezPoint }) =>
  `M ${from.x} ${from.y}
   C ${fromBezPoint.x} ${fromBezPoint.y},
     ${toBezPoint.x} ${toBezPoint.y},
     ${to.x} ${to.y}`;

const bezierControlPoint = (point, mid, anchor, curveShapeFactor) => {
  // NOTE to fellow devs... there is no geometric proof to this math other than
  // the values I selected for curve shape factor did what I wanted on a float from
  // zero to 1. This work was trial and error based =)
  switch (anchor) {
    case TOP:
      return { ...point, y: mid.y - curveShapeFactor * (mid.y / 4) };
    case BOTTOM:
      return { ...point, y: mid.y + curveShapeFactor * (mid.y / 4) };
    case LEFT:
      return { ...point, x: mid.x - curveShapeFactor * (mid.x / 4) };
    case RIGHT:
      return { ...point, x: mid.x + curveShapeFactor * (mid.x / 4) };
    default:
      return null;
  }
};

const pathGeometry = ({ from, to, curveShapeFactor, toAnchor, fromAnchor }) => {
  const mid = {
    x: Math.abs((to.x + from.x) / 2),
    y: Math.abs((to.y + from.y) / 2),
  };
  const fromBezPoint = bezierControlPoint(
    from,
    mid,
    fromAnchor,
    curveShapeFactor
  );
  const toBezPoint = bezierControlPoint(to, mid, toAnchor, curveShapeFactor);

  return {
    to,
    toBezPoint,
    from,
    fromBezPoint,
    mid,
  };
};

const markerGeometry = size => {
  const halfSize = Math.ceil(size / 2);

  return {
    refX: halfSize,
    refY: 0,
    markerWidth: size,
    markerHeight: size,
    viewBox: `0 -${halfSize} ${size} ${size}`,
    path: `M0,-${halfSize} L${size},0L0,${halfSize}`,
  };
};

/**
 * A data flow arrow from the element with id of `from` to that with id of `to`
 *
 */
export class FlowArrow extends LitElement {
  static get styles() {
    return css`
      :host {
        position: absolute;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
      }
    `;
  }

  static get properties() {
    return {
      /**
       * The id of the source element of this arrow.
       * @type {string}
       */
      from: { type: String },

      /**
       * The id of the destination element of this arrow.
       * @type {string}
       */
      to: { type: String },

      /**
       * The anchor position coming from the source element
       * @type {string}
       */
      fromAnchor: { type: String },

      /**
       * The anchor position going to the destination element
       * @type {string}
       */
      toAnchor: { type: String },

      /**
       * The stroke color, defaults to COLOR.
       * @type {string}
       */
      color: { type: String },

      /**
       * The stroke width, defaults to STROKE_WIDTH.
       * @type {number}
       */
      strokeWidth: { type: Number },

      /**
       * Float value between 0.0 and 1 to alter curvature. 0 is a balanced curve and adding to this value creates a "jog back" of the bezier point from the midpoint between to and from.
       * @type {number}
       */
      curveShapeFactor: { type: Number },

      /**
       * Float value between 0.0 and 1 to alter curvature. 0 is a balanced curve and adding to this value creates a "jog back" of the bezier point from the midpoint between to and from.
       * @type {number}
       */
      markerSize: { type: Number },

      /**
       * Offset the terminus of the marker by ? pixels to avoid covering the flow port
       * @type {number}
       */
      offsetMarker: { type: Number, state: true },

      /**
       * Whether this arrow is dashed, defaults to false.
       * @type {boolean}
       */
      dashness: { type: Boolean },

      /**
       * The bounding rect of the source element of this arrow.
       * @type {string}
       */
      fromRect: { type: Object, state: true },

      /**
       * The bounding rect of the destination element of this arrow.
       * @type {string}
       */
      toRect: { type: Object, state: true },

      /**
       * Toggle debug bezier, midpoint, and bounding markers
       * @type {boolean}
       */
      debug: { type: Boolean },
    };
  }

  constructor() {
    super();
    this.fromAnchor = RIGHT;
    this.toAnchor = LEFT;
    this.color = COLOR;
    this.strokeWidth = STROKE_WIDTH;
    this.dashness = false;
    this.curveShapeFactor = CURVE_SHAPE_FACTOR;
    this.markerSize = MARKER_SIZE;
    this.offsetMarker = MARKER_SIZE * 2;
  }

  // Accessors

  set offsetMarker(om) {
    const old = this._offsetMarker;
    this._offsetMarker = om;
    this.requestUpdate('offsetMarker', old);
  }

  get offsetMarker() {
    return this._offsetMarker;
  }

  set fromAnchor(anchor) {
    if (ANCHORS.includes(anchor)) {
      const old = this._fromAnchor;
      this._fromAnchor = anchor;
      this.requestUpdate('fromAnchor', old);
    } else {
      console.warn('fromAnchor must be one of', ANCHORS, anchor);
    }
  }

  get fromAnchor() {
    return this._fromAnchor;
  }

  set toAnchor(anchor) {
    if (ANCHORS.includes(anchor)) {
      const old = this._toAnchor;
      this._toAnchor = anchor;
      this.requestUpdate('toAnchor', old);
    } else {
      console.warn('toAnchor must be one of', ANCHORS, anchor);
    }
  }

  get toAnchor() {
    return this._toAnchor;
  }

  set from(id) {
    const old = this._from;
    this._from = id;
    this.fromRect = this.rectForId(id);
    this.requestUpdate('from', old);
  }

  get from() {
    return this._from;
  }

  set to(id) {
    const old = this._to;
    this._to = id;
    this.toRect = this.rectForId(id);
    this.requestUpdate('to', old);
  }

  get to() {
    return this._to;
  }

  // Lifecycle

  connectedCallback() {
    super.connectedCallback();

    let currentFrom = null;
    let currentTo = null;
    const self = this;
    const timer = setInterval(() => {
      currentFrom = self.fromRect;
      const nextFrom = self.rectForId(self.from);

      currentTo = self.toRect;
      const nextTo = self.rectForId(self.to);

      if (!nextFrom || !nextTo) return;

      if (
        compareRects(currentFrom, nextFrom) &&
        compareRects(currentTo, nextTo)
      ) {
        return;
      }

      self.fromRect = nextFrom;
      self.toRect = nextTo;
    }, DEFAULT_REFRESH_TIME); // TODO: make refresh time configurable via attribute

    this._clear = () => clearInterval(timer);
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    if (typeof this._clear === 'function') {
      this._clear();
    }
  }

  // Domain

  rectForId(id) {
    // DOM Access!!
    const element = document.getElementById(id);
    if (element) {
      return element.getBoundingClientRect();
    }
    return null;
  }

  anchorPoint(anchor, rect, pathHeadOffset = 0, offsetMarker = 0) {
    let x;
    let y;

    switch (anchor) {
      case TOP:
        x = rect.left + Math.ceil((rect.right - rect.left) / 2) + offsetMarker;
        y = rect.top - pathHeadOffset;
        break;
      case LEFT:
        x = rect.left - pathHeadOffset;
        y = rect.top + Math.ceil((rect.bottom - rect.top) / 2) + offsetMarker;
        break;
      case BOTTOM:
        x = rect.left + Math.ceil((rect.right - rect.left) / 2) + offsetMarker;
        y = rect.bottom + pathHeadOffset;
        break;
      case RIGHT:
        x = rect.right + pathHeadOffset;
        y = rect.top + Math.ceil((rect.bottom - rect.top) / 2) + offsetMarker;
        break;
    }

    // DOM access!!
    const container = this.getBoundingClientRect();
    return { x: x - container.x, y: y - container.y };
  }

  buildPathCoords() {
    const {
      fromAnchor,
      toAnchor,
      fromRect,
      toRect,
      curveShapeFactor,
      markerSize,
      offsetMarker,
    } = this;

    const pathHeadOffset = Math.abs(markerSize / 2);
    const from = this.anchorPoint(fromAnchor, fromRect, 0, offsetMarker);
    const to = this.anchorPoint(toAnchor, toRect, pathHeadOffset);

    return pathGeometry({
      to,
      from,
      toAnchor,
      fromAnchor,
      curveShapeFactor,
    });
  }

  // Attributes
  dashArray() {
    return this.dashness ? 6 * this.strokeWidth : null;
  }

  // Render
  renderDashArray() {
    return this.dashness ? html`stroke-dasharray="4"` : html``;
  }

  renderMarker() {
    const { refX, refY, markerWidth, markerHeight, viewBox, path } =
      markerGeometry(this.markerSize);

    return svg`
        <marker id="arrow"
                    viewBox="${viewBox}"
                    refX="${refX}" refY="${refY}"
                    markerWidth="${markerWidth}"
                    markerHeight="${markerHeight}"
                    orient="auto"
                    fill="${this.color}">
          <path d="${path}"></path>
        </marker>
    `;
  }

  renderFlow(pathCoords) {
    return svg`
      <path id="flow-path"
                d="${buildPath(pathCoords)}"
                stroke="${this.color}"
                stroke-width=${this.strokeWidth}
                stroke-dasharray=${ifDefined(this.dashArray())}
                fill="none"
                marker-end="url(#arrow)" />
    `;
  }

  renderDebugOverlay({ to, toBezPoint, mid, fromBezPoint, from }) {
    return this.debug
      ? svg`
        <circle cx="${mid.x}" cy="${mid.y}" r="4" fill="black" ></circle>
        <circle cx="${from.x}" cy="${from.y}" r="4" fill="red" ></circle>
        <circle cx="${fromBezPoint.x}" cy="${fromBezPoint.y}" r="4" fill="blue" ></circle>
        <line x1="${from.x}" y1="${from.y}" x2="${fromBezPoint.x}" y2="${fromBezPoint.y}" stroke="black" stroke-dasharray="4" ></line>
        <circle cx="${toBezPoint.x}" cy="${toBezPoint.y}" r="4" fill="pink" ></circle>
        <circle cx="${to.x}" cy="${to.y}" r="4" fill="blue" ></circle>
        <line x1="${toBezPoint.x}" y1="${toBezPoint.y}" x2="${to.x}" y2="${to.y}" stroke="black" stroke-dasharray="4" ></line>
      `
      : null;
  }

  render() {
    if (this.fromRect && this.toRect) {
      const pathCoords = this.buildPathCoords();

      return html`
        <svg
          width="100%"
          height="100%"
          preserveAspectRatio="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <defs>${this.renderMarker()}</defs>
          ${this.renderDebugOverlay(pathCoords)} ${this.renderFlow(pathCoords)}
        </svg>
      `;
    }

    return html``;
  }
}
