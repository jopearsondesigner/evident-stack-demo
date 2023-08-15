<script lang="ts">
  import { EditorView, basicSetup } from "codemirror"
  import { EditorState } from "@codemirror/state"
  import { createEventDispatcher, onMount } from "svelte";

  export let theme: 'markdown' | 'cue' = 'markdown';
  export let initial_value = '';

  const dispatch = createEventDispatcher();

  const updateListener = EditorView.updateListener.of(function(e) {
    dispatch('document_updated', e);
  });

  let startState = EditorState.create({
    doc: initial_value,
    extensions: [ basicSetup, updateListener ]
  });

  let parent: Element;

  let _view: EditorView;

  onMount(() => {
    _view = new EditorView({
      state: startState,
      parent
    });
  });
</script>

<div class="codemirror-container" bind:this={parent} />
