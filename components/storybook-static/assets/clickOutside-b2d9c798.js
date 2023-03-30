const i=(r,e)=>{const c=t=>{t!=null&&t.target&&r&&!r.contains(t.target)&&!t.defaultPrevented&&e()};return document.addEventListener("click",c,!0),{destroy(){document.removeEventListener("click",c,!0)}}};export{i as c};
//# sourceMappingURL=clickOutside-b2d9c798.js.map
