<script lang="ts">
  import classNames from 'classnames';
  import { EditorView, basicSetup } from 'codemirror';
  import { EditorState } from '@codemirror/state';
  import { createEventDispatcher, onMount } from 'svelte';

  export let theme: 'markdown' | 'cue' = 'markdown';
  export let initial_value = '';
  export let divClass = 'w-full border border-border-light dark:border-border-dark';

  const dispatch = createEventDispatcher();

  const updateListener = EditorView.updateListener.of(function (e) {
    dispatch('document_updated', e);
  });

  let startState = EditorState.create({
    doc: initial_value,
    extensions: [basicSetup, updateListener]
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

<div class={classNames('codemirror-container', divClass)}>
  <slot />
</div>
