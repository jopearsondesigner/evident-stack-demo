<script lang="ts">
  import CodeMirror from 'svelte-codemirror-editor';
  import { javascript } from '@codemirror/lang-javascript';
  import { oneDark } from '@codemirror/theme-one-dark';
  let value = '';

  import Textarea from '$lib/form/Textarea.svelte';
  import Label from '$lib/form/Label.svelte';
  import Highlight, { LineNumbers } from 'svelte-highlight';
  import { HighlightAuto } from 'svelte-highlight';
  import javascriptSyntax from 'svelte-highlight/languages/javascript';
  import 'svelte-highlight/styles/github.css';
  import a11yDark from 'svelte-highlight/styles/a11y-dark';
  let code = '"domain": {\n' + '\tfoo: string, \n' + '\tbar: string, \n' + '\tbaz: int\n' + '}';
</script>

<Label class="mt-8"
  ><span class="mb-1 text-body dark:text-white">Event Model Schema</span>
  <Textarea
    placeholder=""
    value={code}
    name="description"
    rows="6"
    class="mt-1 font-mono block w-full overflow-auto text-sm border border-border-light dark:border-border-dark px-10 py-2.5"
    style="background-color: rgba(48, 56, 65, 100%); color: #D8DEE9;"
    disabled
  />
</Label>

<svelte:head>
  {@html a11yDark}
</svelte:head>

<h2 class="mt-10">Alternative with highlight.js</h2>
<Label class="mt-2 mb-1" color="default">Event Model Schema</Label>
<div
  class="bg-[#303841] text-[#D8DEE9] font-mono block w-full text-sm border border-border-light dark:border-border-dark"
>
  <Highlight language={javascriptSyntax} {code} let:highlighted
    ><LineNumbers {highlighted} /></Highlight
  >
</div>

<Label class="mt-8"><span class="mb-1 text-body dark:text-white">Event Model Schema</span></Label>
<CodeMirror
  bind:value
  theme={oneDark}
  lang={javascript()}
  styles={{
    '&': {
      width: '100%',
      height: '8.875rem',
      backgroundColor: '#303841',
      fontFamily: 'input-mono',
      color: '#D8DEE9'
    }
  }}
  class="mt-1"
/>
