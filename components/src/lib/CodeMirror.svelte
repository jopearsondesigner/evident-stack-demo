<script context="module" lang="ts">
  import { minimalSetup, basicSetup } from 'codemirror';
  import { EditorView, drawSelection, keymap, lineNumbers } from '@codemirror/view';
  import { Transaction, Annotation, EditorState, Compartment } from '@codemirror/state';
  import { undo, redo, defaultKeymap, historyKeymap, history } from '@codemirror/commands';
  export {
    minimalSetup,
    basicSetup,
    EditorView,
    drawSelection,
    keymap,
    lineNumbers,
    Transaction,
    Annotation,
    EditorState,
    Compartment,
    undo,
    redo,
    defaultKeymap,
    historyKeymap,
    history
  };

  export interface EditorItem {
    name: string;
    [id: number]: any;

    // To allow custom keys
    [key: string]: any;
  }
  export type EditorData = EditorItem[];
</script>

<script lang="ts">
  export const editor_data: EditorData = [];

  export const theme: 'markdown' | 'cue' = 'markdown';
  export const initial_value: string | undefined = '';
</script>

<div class="codemirror"><slot /></div>

<style>
  .codemirror {
    width: 100%;
  }
</style>
