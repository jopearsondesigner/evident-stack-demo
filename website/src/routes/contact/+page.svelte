<script lang='ts'>
    import Container from 'components/Container.svelte';
    import Row from 'components/Row.svelte';
    import Column from 'components/Column.svelte';
    import Section from 'components/Column.svelte';
    import SectionHeader from 'components/typography/SectionHeader.svelte';
    import SectionCopy from 'components/typography/SectionCopy.svelte';
    import SectionLabel from 'components/typography/SectionLabel.svelte';
    import Icon from 'components/Icon.svelte';
    import Pin from 'components/icons/Pin.svelte';
    import Phone from 'components/icons/Phone.svelte';
    import Email from 'components/icons/Email.svelte';
</script>

<Container class='sm:container pb-4'>
    <SectionLabel>
        Contact us
    </SectionLabel>
</Container>
<Container class='sm:container w-full h-auto relative'>
    <Row class='grid px-4' customCols='lg:grid-cols-6'>
        <Column customSpan='lg:col-start-2 col-span-4'>
            <Section name='general'>
                <SectionHeader tag='h2' hClass='leading-[1.27] font-bold font-header mb-4 text-center' customSize='lg:text-[30px] text-[24px]' color='text-gray-brand-2 dark:text-gray-brand-4'>
                    Have a question for us?
                </SectionHeader>
            </Section>
        </Column>
    </Row>
</Container>
<Container class='sm:container w-full h-auto relative'>
    <Row class='grid px-4 my-24' customCols='lg:grid-cols-2'>
        <Column class='flex justify-center items-center bg-stack-brand-800 lg:p-4 px-4 py-8'>
            <address>
                <ul class="list-none">
                    <li class='my-3'>
                        <div class="text-white inline-block pointer-events-none">
                            <span class="relative float-left align-middle mt-[3px] mr-2">
                                <Icon name='Pin' iconColor='text-white'><Pin/></Icon>
                            </span>
                            <span class="text-base">12110 Sunset Hills Rd #600 Reston, VA 20190</span>
                        </div>
                    </li>
                    <li class='my-3'>
                        <a href="tel:+13115552368" class="text-white inline-block">
                            <span class="relative float-left align-middle mt-[3px] mr-2">
                                <Icon name='Phone' iconColor='text-white'><Phone/></Icon>
                            </span>
                            <span class="text-base">(703)-259-1199</span>
                        </a>
                    </li>
                    <li class='my-3'>
                        <a href="mailto:hello@onote.com" class="text-white inline-block">
                            <span class="relative float-left align-middle mt-[3px] mr-2">
                                <Icon name='Email' iconColor='text-white'><Email/></Icon>
                            </span>
                            <span class="text-base">hello@onote.com</span>
                        </a>
                    </li>
                </ul>
            </address>
        </Column>
        <Column class='lg:order-last order-first ml-4 mt-4'>
            <!--[if lte IE 8]>
            <script charset="utf-8" type="text/javascript" src="//js.hsforms.net/forms/v2-legacy.js"></script>
            <![endif]-->
            <script charset="utf-8" type="text/javascript" src="//js.hsforms.net/forms/v2.js"></script>
            <script>
                hbspt.forms.create({
                portalId: "4640583",
                formId: "62679224-882c-40ae-afb4-9c58ba11b471",
                css: "",
                cssClass: "contact",
                submitButtonClass: "mx-2 text-sm rounded-md inline-flex items-center px-4 py-1 text-white hover:text-white transition-colors duration-300 ease-in-out bg-gradient-to-b from-primary to-secondary-red hover:bg-gradient-to-b hover:from-secondary-red hover:to-secondary-red uppercase no-underline",
                onFormSubmit: function ($form) { // YOUR SCRIPT HERE
                }
                });
            </script>
        </Column>
    </Row>
</Container>

