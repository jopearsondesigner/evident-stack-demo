<script lang="ts">
  import type { PageData } from './$types';
  import CodeMirror, {
    EditorView,
    Compartment,
    EditorState,
    basicSetup,
    history,
    drawSelection,
    lineNumbers,
    keymap,
    defaultKeymap,
    historyKeymap,
    undo,
    redo,
    Annotation,
    Transaction
  } from '$components/CodeMirror.svelte';
  import Button from '$components/Button.svelte';
  import TabWrapper from '$components/tabs/TabWrapper.svelte';
  import TabHead from '$components/tabs/TabHead.svelte';
  import TabHeadItem from '$components/tabs/TabHeadItem.svelte';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { javascript } from '@codemirror/lang-javascript';
  import { markdown } from '@codemirror/lang-markdown';
  import { languages } from '@codemirror/language-data';
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import ThemeSwitch from '$components/utils/ThemeSwitch.svelte';

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
    initial_value: string = 'Hello jo';

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

  $: editor_data = [
    { name: 'Vehicle Added', id: 1 },
    { name: 'Vehicle Removed/Placement', id: 2 },
    { name: 'Add a Vehicle/Placement', id: 3 }
  ];

  export let activeTabValue: number = 1;

  export const handleClick = (tabValue: number) => () => {
    activeTabValue = tabValue;
  };
</script>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-0"><ThemeSwitch /></span>

<div class="h-screen w-full flex justify-center items-center">
  <div class="w-full" bind:this={parent}>
    <CodeMirror>
      <TabWrapper divClass=" bg-dark-2 h-[492px] flex w-full relative">
        <TabHead divClass="h-[29px] w-full self-start ml-[30px] pt-px">
          {#each editor_data as editor (editor.id)}
            <TabHeadItem
              id={editor.id}
              {activeTabValue}
              name={editor.name}
              on:click={handleClick(editor.id)}
            >
              <slot {editor} list={editor_data} id={editor.id}>
                {editor.name}
              </slot>
            </TabHeadItem>
          {/each}
        </TabHead>
        <div class="flex w-full absolute top-[29px] bottom-[53px] h-auto max-h-full">
          <div id="editor" class="w-1/2" />
          <div id="output" class="w-1/2" />
        </div>
        <div class="h-[53px] flex justify-start items-center w-full absolute bottom-0">
          <div class="flex items-center justify-end space-x-2 px-2 w-1/2">
            <Button
              ghostTextColor="text-white"
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
</div>
