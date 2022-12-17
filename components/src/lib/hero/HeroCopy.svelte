<script lang='ts'>
    export let heroCopyClass='lg:mt-0 mt-8 sm:mt-[-130px] mt-[-103px] mb-16 lg:ml-16';
    export let headerClass='text-gray-brand-2 dark:text-gray-brand-4 2xl:text-[62px] xl:text-[62px] lg:text-[36px] sm:text-[50px] text-[30px] leading-[1.16] font-hero font-bold mb-4 lg:pl-4 lg:text-left text-center';
    export let copyClass='text-[18px] leading-normal lg:pl-4 pb-4 lg:text-left text-center';
    export let ctaClass='px-4 lg:justify-start flex justify-center';
</script>

<div class={heroCopyClass}>
    <h1 id="heroHeader" class={headerClass}>
        <slot name='hero-header'/>
    </h1>
    <div class={copyClass}>
        <slot name='hero-copy'/>
    </div>
    <div class={ctaClass}>
        <slot name='hero-cta'/>
    </div>
</div>
