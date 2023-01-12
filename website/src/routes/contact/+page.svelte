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
    import Form from 'components/form/Form.svelte';
    import Input from 'components/form/Input.svelte';
    import Textarea from 'components/form/Textarea.svelte';
    import Label from 'components/form/Label.svelte';
    import A from 'components/typography/A.svelte';
    import Submit from 'components/form/Submit.svelte';
</script>

<Container class='sm:container pb-4'>
    <Section name='general' class='flex items-center px-4'>
        <SectionLabel>
            Contact us
        </SectionLabel>
    </Section>
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
    <Row class='grid px-4 lg:mt-24 mb-24' customCols='lg:grid-cols-2'>
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
        <Column class='lg:order-last order-first'>
            <div class="w-full p-4">
                <Form action="create_ticket" method="post">
                    <ul>
                        <li class="mb-4">
                            <Input type='text' placeholder="ticket subject"
                                name='subject' class='border' required/>
                        </li>
                        <li class="mb-4">
                            <Textarea placeholder="what's the problem?" name='description' rows='6' class='' required/>
                        </li>
                        <li class="mb-4">
                            <Input type='email' placeholder="your email address" name='email' class='border' required/>
                            <div class="text-xs"><A href='https://evidentsystems.zendesk.com/auth/v2/login/registration' target='_blank' class='transition duration-300 ease-in'>Register</A> <span class="text-body-light dark:text-body-dark">so our support team can email you to solve your problem.</span></div>
                        </li>
                        <li class="mb-4">
                            <Submit type='submit' value='Submit' gradient color='brandStackPrimary' size='sm'/>
                        </li>
                    </ul>
                </Form>
            </div>
        </Column>
    </Row>
</Container>

