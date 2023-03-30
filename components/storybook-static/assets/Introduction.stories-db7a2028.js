import{r as m,M as h}from"./chunk-PCJTTTQV-d621b9db.js";import"./chunk-R4NKYYJA-96bb58e6.js";import{u as d}from"./index-10aa761b.js";import"./iframe-23539328.js";import"../sb-preview/runtime.mjs";import"./_commonjsHelpers-725317a4.js";import"./index-d475d2ea.js";import"./index-d37d4223.js";import"./index-88e936b2.js";import"./index-356e4a49.js";const g=""+new URL("code-brackets-9ef6443e.svg",import.meta.url).href,x=""+new URL("colors-ac9401f3.svg",import.meta.url).href,u=""+new URL("comments-f15a6837.svg",import.meta.url).href,f=""+new URL("direction-94a9917f.svg",import.meta.url).href,j=""+new URL("flow-275142c6.svg",import.meta.url).href,k=""+new URL("plugin-57148314.svg",import.meta.url).href,b=""+new URL("repo-fb4ece47.svg",import.meta.url).href,y=""+new URL("stackalt-2ad81543.svg",import.meta.url).href;var e={},w={get exports(){return e},set exports(s){e=s}},a={};/**
 * @license React
 * react-jsx-runtime.production.min.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var _=m,v=Symbol.for("react.element"),S=Symbol.for("react.fragment"),E=Object.prototype.hasOwnProperty,N=_.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED.ReactCurrentOwner,R={key:!0,ref:!0,__self:!0,__source:!0};function p(s,r,n){var t,o={},l=null,c=null;n!==void 0&&(l=""+n),r.key!==void 0&&(l=""+r.key),r.ref!==void 0&&(c=r.ref);for(t in r)E.call(r,t)&&!R.hasOwnProperty(t)&&(o[t]=r[t]);if(s&&s.defaultProps)for(t in r=s.defaultProps,r)o[t]===void 0&&(o[t]=r[t]);return{$$typeof:v,type:s,key:l,ref:c,props:o,_owner:N.current}}a.Fragment=S;a.jsx=p;a.jsxs=p;(function(s){s.exports=a})(w);function C(s={}){const{wrapper:r}=Object.assign({},d(),s.components);return r?e.jsx(r,{...s,children:e.jsx(n,{})}):n();function n(){const t=Object.assign({h1:"h1",p:"p",strong:"strong",code:"code",a:"a"},d(),s.components);return e.jsxs(e.Fragment,{children:[e.jsx(h,{title:"Example/Introduction"}),`
`,e.jsx("style",{children:`
    .subheading {
      --mediumdark: '#999999';
      font-weight: 900;
      font-size: 13px;
      color: #999;
      letter-spacing: 6px;
      line-height: 24px;
      text-transform: uppercase;
      margin-bottom: 12px;
      margin-top: 40px;
    }

    .link-list {
      display: grid;
      grid-template-columns: 1fr;
      grid-template-rows: 1fr 1fr;
      row-gap: 10px;
    }

    @media (min-width: 620px) {
      .link-list {
        row-gap: 20px;
        column-gap: 20px;
        grid-template-columns: 1fr 1fr;
      }
    }

    @media all and (-ms-high-contrast:none) {
    .link-list {
        display: -ms-grid;
        -ms-grid-columns: 1fr 1fr;
        -ms-grid-rows: 1fr 1fr;
      }
    }

    .link-item {
      display: block;
      padding: 20px 30px 20px 15px;
      border: 1px solid #00000010;
      border-radius: 5px;
      transition: background 150ms ease-out, border 150ms ease-out, transform 150ms ease-out;
      color: #333333;
      display: flex;
      align-items: flex-start;
    }

    .link-item:hover {
      border-color: #1EA7FD50;
      transform: translate3d(0, -3px, 0);
      box-shadow: rgba(0, 0, 0, 0.08) 0 3px 10px 0;
    }

    .link-item:active {
      border-color: #1EA7FD;
      transform: translate3d(0, 0, 0);
    }

    .link-item strong {
      font-weight: 700;
      display: block;
      margin-bottom: 2px;
    }

    .link-item img {
      height: 40px;
      width: 40px;
      margin-right: 15px;
      flex: none;
    }

    .link-item span {
      font-size: 14px;
      line-height: 20px;
    }

    .tip {
      display: inline-block;
      border-radius: 1em;
      font-size: 11px;
      line-height: 12px;
      font-weight: 700;
      background: #E7FDD8;
      color: #66BF3C;
      padding: 4px 12px;
      margin-right: 10px;
      vertical-align: top;
    }

    .tip-wrapper {
      font-size: 13px;
      line-height: 20px;
      margin-top: 40px;
      margin-bottom: 40px;
    }

    .tip-wrapper code {
      font-size: 12px;
      display: inline-block;
    }
  `}),`
`,e.jsx(t.h1,{children:"Welcome to Storybook"}),`
`,e.jsxs(t.p,{children:[`Storybook helps you build UI components in isolation from your app's business logic, data, and context.
That makes it easy to develop hard-to-reach states. Save these UI states as `,e.jsx(t.strong,{children:"stories"})," to revisit during development, testing, or QA."]}),`
`,e.jsxs(t.p,{children:[`Browse example stories now by navigating to them in the sidebar.
View their code in the `,e.jsx(t.code,{children:"stories"}),` directory to learn how they work.
We recommend building UIs with a `,e.jsx(t.a,{href:"https://componentdriven.org",children:e.jsx(t.strong,{children:"component-driven"})})," process starting with atomic components and ending with pages."]}),`
`,e.jsx("div",{className:"subheading",children:"Configure"}),`
`,e.jsxs("div",{className:"link-list",children:[e.jsxs("a",{className:"link-item",href:"https://storybook.js.org/docs/react/addons/addon-types",target:"_blank",children:[e.jsx("img",{src:k,alt:"plugin"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"Presets for popular tools"}),`
Easy setup for TypeScript, SCSS and more.`]})})]}),e.jsxs("a",{className:"link-item",href:"https://storybook.js.org/docs/react/configure/webpack",target:"_blank",children:[e.jsx("img",{src:y,alt:"Build"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"Build configuration"}),`
How to customize webpack and Babel`]})})]}),e.jsxs("a",{className:"link-item",href:"https://storybook.js.org/docs/react/configure/styling-and-css",target:"_blank",children:[e.jsx("img",{src:x,alt:"colors"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"Styling"}),`
How to load and configure CSS libraries`]})})]}),e.jsxs("a",{className:"link-item",href:"https://storybook.js.org/docs/react/get-started/setup#configure-storybook-for-your-stack",target:"_blank",children:[e.jsx("img",{src:j,alt:"flow"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"Data"}),`
Providers and mocking for data libraries`]})})]})]}),`
`,e.jsx("div",{className:"subheading",children:"Learn"}),`
`,e.jsxs("div",{className:"link-list",children:[e.jsxs("a",{className:"link-item",href:"https://storybook.js.org/docs",target:"_blank",children:[e.jsx("img",{src:b,alt:"repo"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"Storybook documentation"}),`
Configure, customize, and extend`]})})]}),e.jsxs("a",{className:"link-item",href:"https://storybook.js.org/tutorials/",target:"_blank",children:[e.jsx("img",{src:f,alt:"direction"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"In-depth guides"}),`
Best practices from leading teams`]})})]}),e.jsxs("a",{className:"link-item",href:"https://github.com/storybookjs/storybook",target:"_blank",children:[e.jsx("img",{src:g,alt:"code"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"GitHub project"}),`
View the source and add issues`]})})]}),e.jsxs("a",{className:"link-item",href:"https://discord.gg/storybook",target:"_blank",children:[e.jsx("img",{src:u,alt:"comments"}),e.jsx("span",{children:e.jsxs(t.p,{children:[e.jsx("strong",{children:"Discord chat"}),`
Chat with maintainers and the community`]})})]})]}),`
`,e.jsx("div",{className:"tip-wrapper",children:e.jsxs(t.p,{children:[e.jsx("span",{className:"tip",children:"Tip"}),"Edit the Markdown in ",e.jsx("code",{children:"stories/Introduction.stories.mdx"})]})})]})}}const D=()=>{throw new Error("Docs-only story")};D.parameters={docsOnly:!0};const i={title:"Example/Introduction",tags:["stories-mdx"],includeStories:["__page"]};i.parameters=i.parameters||{};i.parameters.docs={...i.parameters.docs||{},page:C};const A=["__page"];export{A as __namedExportsOrder,D as __page,i as default};
//# sourceMappingURL=Introduction.stories-db7a2028.js.map
