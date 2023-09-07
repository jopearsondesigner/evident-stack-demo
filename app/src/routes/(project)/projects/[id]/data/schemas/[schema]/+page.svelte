<script lang="ts">
  import CodeMirror from '$components/CodeMirror.svelte';
  import { EditorView, basicSetup } from 'codemirror';
  import { EditorState, Compartment, Annotation, Transaction } from '@codemirror/state';
  import { defaultKeymap, historyKeymap, history, undo, redo } from '@codemirror/commands';
  import { drawSelection, keymap, lineNumbers } from '@codemirror/view';

  import Button from '$components/Button.svelte';
  import TabWrapper from '$components/tabs/TabWrapper.svelte';
  import TabHead from '$components/tabs/TabHead.svelte';
  import TabHeadItem from '$components/tabs/TabHeadItem.svelte';
  import { javascript } from '@codemirror/lang-javascript';
  import { markdown } from '@codemirror/lang-markdown';
  import { languages } from '@codemirror/language-data';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { page } from '$app/stores';

  export let _mounted = false;
  onMount(() => {
    _mounted = true;
    return () => {
      _mounted = false;
    };
  });

  let evidentTheme = EditorView.theme({
    '&': {
      width: '100%',
      height: '100%',
      backgroundColor: '#303841',
      fontSize: '13px',
      color: '#D8DEE9'
    },
    '.cm-editor': { fontFamily: 'input-mono' },
    '.cm-content': { fontFamily: 'input-mono' },
    '.cm-gutters': {
      backgroundColor: '#282C34'
    }
  });

  let mainView: EditorView;
  let otherView: EditorView;

  const dispatch = createEventDispatcher();
  const updateListener = EditorView.updateListener.of(function (e) {
    dispatch('document_updated', e);
  });

  let language = new Compartment(),
    tabSize = new Compartment(),
    parent: Element | DocumentFragment | undefined,
    initial_value: string = '';

  let startState = EditorState.create({
    doc: initial_value,
    extensions: [
      basicSetup,
      language.of(javascript()),
      evidentTheme,
      oneDark,
      updateListener,
      history(),
      drawSelection(),
      lineNumbers(),
      keymap.of([...defaultKeymap, ...historyKeymap])
    ]
  });

  let otherState = EditorState.create({
    doc: startState.doc,
    extensions: [
      basicSetup,
      markdown({ codeLanguages: languages }),
      evidentTheme,
      oneDark,
      updateListener,
      drawSelection(),
      lineNumbers(),
      keymap.of([
        ...defaultKeymap,
        { key: 'Mod-z', run: () => undo(mainView) },
        { key: 'Mod-y', mac: 'Mod-Shift-z', run: () => redo(mainView) }
      ])
    ]
  });

  let syncAnnotation = Annotation.define<boolean>();

  function syncDispatch(tr: Transaction, view: EditorView, other: EditorView) {
    view.update([tr]);
    if (!tr.changes.empty && !tr.annotation(syncAnnotation)) {
      let annotations: Annotation<any>[] = [syncAnnotation.of(true)];
      let userEvent = tr.annotation(Transaction.userEvent);
      if (userEvent) annotations.push(Transaction.userEvent.of(userEvent));
      other.dispatch({ changes: tr.changes, annotations });
    }
  }

  $: if (_mounted) {
    mainView = new EditorView({
      state: startState,
      // @ts-ignore
      parent: document.querySelector('#editor'),
      dispatch: (tr) => syncDispatch(tr, mainView, otherView)
      // setState: EditorState.create({ doc: 'my new content' })
    });
    otherView = new EditorView({
      state: otherState,
      // @ts-ignore
      parent: document.querySelector('#output'),
      dispatch: (tr) => syncDispatch(tr, otherView, mainView)
    });
  }

  $: onDestroy(() => {
    if (mainView !== null) {
      mainView.destroy();
    }
    if (otherView !== null) {
      otherView.destroy();
    }
  });

  // Data for testing
  $: tree_data = [
    {
      name: 'Autonomo Mobile iOS App',
      type: 'event-model',
      id: 1
    },
    {
      name: 'My Vehicles',
      type: 'read-model',
      id: 2
    },
    {
      name: 'Vehicle Added',
      type: 'event',
      id: 3
    },
    {
      name: 'Vehicle Added/Placement',
      type: 'placement',
      id: 4
    },
    {
      name: 'Change',
      type: 'event',
      id: 5
    },
    {
      name: 'Change/Placement',
      type: 'placement',
      id: 6
    },
    {
      name: 'Vehicle Removed',
      type: 'event',
      id: 7
    },
    {
      name: 'Vehicle Removed/Placement',
      type: 'placement',
      id: 8
    },
    {
      name: 'Add a Vehicle',
      type: 'interface',
      id: 9
    },
    {
      name: 'Add a Vehicle/Placement',
      type: 'placement',
      id: 10
    },
    {
      name: 'My Vehicles',
      type: 'interface',
      id: 11
    },
    {
      name: 'My Vehicles/Placement',
      type: 'placement',
      id: 12
    },
    {
      name: 'Remove a Vehicle',
      type: 'interface',
      id: 13
    },
    {
      name: 'Remove a Vehicle/Placement',
      type: 'placement',
      id: 14
    },
    {
      name: 'Add a Vehicle',
      type: 'command',
      id: 15
    },
    {
      name: 'Vehicle Added',
      type: 'event',
      id: 16
    },
    {
      name: 'Vehicle Added/Placement',
      type: 'placement',
      id: 17
    },
    {
      name: 'Add a Vehicle',
      type: 'interface',
      id: 18
    },
    {
      name: 'Add a Vehicle/Placement',
      type: 'placement',
      id: 19
    }
  ];
</script>

<div class="h-screen bg-gray-canvas dark:bg-dark-1 w-full flex justify-center items-center px-3">
  <CodeMirror>
    <TabWrapper divClass="bg-dark-2 dark:bg-dark-2 h-[492px] flex w-full relative">
      <TabHead divClass="h-[29px] w-full self-start pl-[30px] pt-px">
        {#each tree_data as item (item.id)}
          <TabHeadItem
            id={item.id}
            name={item.name}
            href="/projects/{$page.params.id}/data/schemas/{item.id}"
            on:click
          />
        {/each}
      </TabHead>
      <div class="flex w-full absolute top-[29px] bottom-[53px] h-auto max-h-full">
        <div id="editor" class="w-1/2" bind:this={parent} />
        <div id="output" class="w-1/2" bind:this={parent} />
      </div>
      <div
        class="h-[53px] flex justify-start items-center w-full absolute bottom-0 border-t border-border-dark dark:border-border-dark"
      >
        <div class="flex items-center justify-end space-x-2 px-2 w-1/2">
          <Button
            ghostTextColor="text-white dark:text-white"
            gradient
            color="ghost"
            size="sm"
            class="flex-none"
            label="Cancel"
            on:click
          />
          <Button color="default" size="sm" class="flex-none" label="Save" on:click />
        </div>
      </div>
    </TabWrapper>
  </CodeMirror>
</div>
