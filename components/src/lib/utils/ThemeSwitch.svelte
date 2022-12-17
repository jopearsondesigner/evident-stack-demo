<script lang='ts'>
	import { useState } from 'react';
    export let checked = false;
    import Icon from '../Icon.svelte';
    import ThemeSwitchIcon from '../icons/ThemeSwitchIcon.svelte';
    export const themeSwitchClass:string='';

    // Don't use this one
    // const [darkMode,setDarkMode] = useState(false);
    // const toggleTheme = ()=>{
    //     setDarkMode(!darkMode);
    //     const root = window.document.documentElement; //add 'dark class to html root element'
    //     root.classList.toggle('dark');
    // }

      // USE THIS ONE
    // import { browser } from '../../../node_modules/@sveltejs/kit/src/runtime/app/environment.js';

    // let darkMode = true;

    // function handleSwitchDarkMode() {
    //     darkMode = !darkMode;

    //     localStorage.setItem('theme', darkMode ? 'dark' : 'light');

    //     darkMode
    //         ? document.documentElement.classList.add('dark')
    //         : document.documentElement.classList.remove('dark');
    // }

    // if (browser) {
    //     if (
    //         localStorage.theme === 'dark' ||
    //         (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)
    //     ) {
    //         document.documentElement.classList.add('dark');
    //         darkMode = true;
    //     } else {
    //         document.documentElement.classList.remove('dark');
    //         darkMode = false;
    //     }
    // }
  </script>

  <style>
    .switch {
      position: relative;
      display: inline-block;
      width: 35px;
      height: 20px;
    }

    .switch input {
      opacity: 0;
      width: 0;
      height: 0;
    }

    .slider {
      position: absolute;
      cursor: pointer;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background-color: #ccc;
      border: 1px solid #93A1A1;
      -webkit-transition: 0.4s;
      transition: 0.4s;
      border-radius: 34px;
    }

    .slider:before {
      position: absolute;
      content: "";
      height: 15px;
      width: 15px;
      left: 2px;
      bottom: 1.5px;
      background-color: white;
      -webkit-transition: 0.4s;
      transition: 0.4s;
      border-radius: 50%;
    }

    input:checked + .slider {
      background-color: #002B36;
    }

    input:checked + .slider {
      box-shadow: 0 0 1px #002B36;
    }

    input:checked + .slider:before {
      -webkit-transform: translateX(14px);
      -ms-transform: translateX(14px);
      transform: translateX(14px);
    }
  </style>

  <!-- <label class="switch" for="theme-toggle">
    <input checked={darkMode} on:click={handleSwitchDarkMode} type="checkbox" id="theme-toggle" />
    <span class="slider">
        <Icon name='mode-switch-icon' size={37} iconColor='fill-current' viewBox='0 0 60 60' class='-mt-[9px]'><ThemeSwitchIcon/></Icon>
    </span>
  </label> -->

  <label class="switch" for="theme-toggle">
    <input type="checkbox" id="theme-toggle" />
    <span class="slider">
        <Icon name='theme-switch-icon' size={37} iconColor='fill-current' viewBox='0 0 60 60' class='-mt-[9px]'><ThemeSwitchIcon/></Icon>
    </span>
  </label>




