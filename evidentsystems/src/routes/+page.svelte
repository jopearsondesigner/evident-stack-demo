<script lang="ts">
  import classNames from 'classnames';
  import Player from '$components/utils/Player.svelte';

  import Carousel from 'svelte-carousel';
  import { browser } from '$app/environment';

  let carousel: { goToNext: () => void }; // for calling methods of the carousel instance

  const handleNextClick = () => {
    carousel.goToNext();
  };

  import Hero from '$components/hero/Hero.svelte';
  import HeroContainer from '$components/hero/HeroContainer.svelte';
  import HeroCopy from '$components/hero/HeroCopy.svelte';
  import HeroFlexbox from '$components/hero/HeroFlexbox.svelte';
  import HeroGrid from '$components/hero/HeroGrid.svelte';
  import HeroImage from '$components/hero/HeroImage.svelte';
  import Pin from '$components/icons/Pin.svelte';
  import Phone from '$components/icons/Phone.svelte';
  import Email from '$components/icons/Email.svelte';
  import Form from '$components/form/Form.svelte';
  import Input from '$components/form/Input.svelte';
  import Textarea from '$components/form/Textarea.svelte';
  import Submit from '$components/form/Submit.svelte';
  import Button from '$components/Button.svelte';
  import HeroGraphic from '$components/assets/images/evidentsystems/hero-graphic.svg';
  import ForegroundImage from '$components/assets/images/homepage/hero/heroLaptop.svg';
  export let heroFlexboxClass = 'flex justify-center items-center';
  export let gradientDividerClass =
    'h-[64px] bg-gradient-to-t from-gradient to-transparency opacity-20';
  import Container from '$components/Container.svelte';
  import SectionLabel from '$components/typography/SectionLabel.svelte';
  import Section from '$components/Section.svelte';
  import SectionHeader from '$components/typography/SectionHeader.svelte';
  import SectionCopy from '$components/typography/SectionCopy.svelte';
  import A from '$components/typography/A.svelte';
  import Row from '$components/Row.svelte';
  import Column from '$components/Column.svelte';
  import PoorSoftwareQuality from '$components/assets/images/homepage/poorSoftwareQuality.svg';
  import PoorSoftwareQualityDark from '$components/assets/images/homepage/poorSoftwareQualityDark.svg';
  let mainClass = 'bg-white w-full h-auto z-10 relative pb-[128px]';
  export let gridClass: string = 'grid lg:grid-cols-2 grid-col-1 mt-[0px]';
  export let flexClass: string = 'flex justify-center items-center px-4';
  export let containerClass: string | undefined = 'container pb-16';
  import Nubank from '$components/assets/images/evidentsystems/nubank.png';
  import Toyota from '$components/assets/images/evidentsystems/toyota.png';
  import Confluent from '$components/assets/images/evidentsystems/confluent.png';
  import HRBlock from '$components/assets/images/evidentsystems/hrblock.png';
  import Parkside from '$components/assets/images/evidentsystems/parkside.png';
  import Reify from '$components/assets/images/evidentsystems/reify.png';
  import EventStore from '$components/assets/images/evidentsystems/eventstore.png';
  import Recursion from '$components/assets/images/evidentsystems/recursion.png';
  import SeventyFive from '$components/assets/images/homepage/infoSeventyFive.svg';
  import SeventyEight from '$components/assets/images/homepage/infoSeventyEight.svg';
  import Eighty from '$components/assets/images/homepage/infoEighty.svg';
  import FifyFive from '$components/assets/images/homepage/infoFiftyFive.svg';
  import SeventyFiveDark from '$components/assets/images/homepage/infoSeventyFiveDark.svg';
  import SeventyEightDark from '$components/assets/images/homepage/infoSeventyEightDark.svg';
  import EightyDark from '$components/assets/images/homepage/infoEightyDark.svg';
  import FifyFiveDark from '$components/assets/images/homepage/infoFiftyFiveDark.svg';
  import GradientDivider from '$components/utils/GradientDivider.svelte';
  import Heart from '$components/icons/Heart.svelte';
  import Docs from '$components/icons/Docs.svelte';
  import Icon from '$components/Icon.svelte';
  import ArrowRight from '$components/icons/ArrowRight.svelte';
  import ArrowLeft from '$components/icons/ArrowLeft.svelte';
  import Logo from '$components/assets/images/global/evidentStackLogo.svg';
  import BackgroundImg from '$components/assets/images/evidentsystems/background-img-1.jpg';
  import Navbar from '$components/navbar/Navbar.svelte';
  import NavWrapper from '$components/navbar/NavWrapper.svelte';
  import NabBrand from '$components/navbar/NavBrand.svelte';
  let website: boolean = true;

  import { MetaTags } from 'svelte-meta-tags';
  let title = 'Evident Stack';
  let description =
    'Each layer in the Stack can be used individually, but work best together since each layer builds upon the one before it.';
</script>

<section
  class="h-full text-body-dark bg-center bg-no-repeat bg-contain bg-[#000003]"
  style="background-image: url({BackgroundImg})"
>
  <HeroContainer>
    <HeroGrid>
      <HeroFlexbox class={classNames('lg:mt-[56px] relative z-10 lg:px-0 px-4', heroFlexboxClass)}>
        <HeroCopy
          heroCopyClass="mb-16 lg:ml-16"
          headerClass="text-gray-brand-4 font-primary font-bold leading-normal 2xl:text-[56px] xl:text-[56px] lg:text-[36px] sm:text-[50px] text-[30px] lg:pl-4 lg:text-left text-center"
        >
          <span slot="hero-header">Improve your Enterprise Software Development by 100%</span>
          <span slot="hero-copy" class="text-body-dark"
            >Reduce upfront cost and risk. Build systems that produce better data.</span
          >

          <Button
            slot="hero-cta"
            color="evidentsystems"
            class="flex mt-2 "
            size="md"
            href="#contact"
            label="Contact"
          />
        </HeroCopy>
      </HeroFlexbox>
      <HeroFlexbox
        class={classNames(
          'flex-row lg:order-last order-first mx-auto pt-0 sm:pt-4',
          heroFlexboxClass
        )}
      >
        <HeroImage
          imgSize="2xl:w-[640px] xl:w-[570px] lg:w-[440px] md:w-[540px] sm:w-[540px] sm:mt-0 mt-[16px] flex justify-center items-center m-8"
        >
          <img
            src={HeroGraphic}
            alt="Enterprise Software Development Graph"
            width="100%"
            height="100%"
            class="table-row"
          />
        </HeroImage>
      </HeroFlexbox>
    </HeroGrid>
  </HeroContainer>
  <GradientDivider class={classNames('-mt-16', gradientDividerClass)} />
</section>

<main class={mainClass}>
  <Container class={classNames(containerClass)}>
    <SectionLabel>Trusted by</SectionLabel>
    <Row
      class="grid lg:grid-cols-4 md:grid-cols-3 sm:grid-cols-3 grid-cols-2 gap-8 content-center mt-[44px] py-20"
    >
      <Column class="flex justify-center self-center">
        <img src={Nubank} alt="Nubank" width="18%" />
      </Column>
      <Column class="flex justify-center self-center">
        <img src={Toyota} alt="Toyota" width="23%" />
      </Column>
      <Column class="flex justify-center self-center">
        <img src={Confluent} alt="Confluent" width="60%" />
      </Column>
      <Column class="flex justify-center self-center">
        <img src={HRBlock} alt="H&R Block" width="23%" />
      </Column>
      <Column class="flex justify-center self-center">
        <img src={Parkside} alt="Parkside" width="60%" />
      </Column>
      <Column class="flex justify-center self-center">
        <img src={Reify} alt="Reify Health" width="60%" />
      </Column>
      <Column class="flex justify-center self-center">
        <img src={EventStore} alt="Event Store" width="60%" />
      </Column>
      <Column class="flex justify-center self-center">
        <img src={Recursion} alt="Recursion" width="60%" />
      </Column>
    </Row>
  </Container>
  <div class="bg-black bg-opacity-[4%]">
    <Container class={classNames('py-[60px]', containerClass)}>
      <Row class={classNames('lg:my-[44px] lg:py-[69px]', gridClass)}>
        <Column class="flex justify-center items-center lg:order-last mx-auto mt-8 lg:mt-0 px-4">
          <Player src="lottie/teamCollaborationHero.json" />
        </Column>
        <Column class={classNames('order-first lg:pr-12 lg:-mt-4 px-4 flex items-center')}>
          <Section name="benefit">
            <img
              src={Logo}
              alt="Evident Stack logo"
              height="auto"
              style="height:48px;"
              class="ml-4 mb-12"
            />
            <SectionHeader
              tag="h2"
              color="text-gray-brand-1"
              hClass="font-primary font-extrabold mb-4 lg:pl-4 lg:text-left text-center leading-[1.27]"
              customSize="text-[2rem]"
            >
              Take a look at our latest product
            </SectionHeader>
            <SectionCopy class="lg:pl-4 lg:text-left text-center mb-4">
              Evident Stack is a new platform we’ve created to leverage the latest event modeling
              technology. It allows your team to collaborate online and helps you make your
              company’s information systems Evident.
            </SectionCopy>
            <span class="flex ml-4 mt-8 lg:justify-start justify-center">
              <Button
                gradient
                color="brandStackPrimary"
                size="sm"
                label="Check Out Evident Stack"
                class="flex items-center flex-none"
                href="https://evidentstack.com/"
              >
                <Icon
                  name="arrow-right"
                  slot="icon"
                  class="inline-flex ml-1 stroke-2"
                  iconColor="fill-current text-white"
                  size={12}
                  pathName={ArrowRight}
                />
              </Button>
            </span>
          </Section>
        </Column>
      </Row>
    </Container>
    <GradientDivider class="=mt=16" />
  </div>

  <Container>
    <span id="contact">
      <SectionLabel class="text-gray-primary font-bold py-4 ml-4 text-left">Contact</SectionLabel>
      <Row class="grid px-4 lg:mt-8 mb-8" customCols="lg:grid-cols-2">
        <Column class="flex justify-center items-center lg:p-4 px-4 py-8">
          <address>
            <ul class="list-none">
              <li class="my-3">
                <div class="text-body-light inline-block pointer-events-none">
                  <span class="relative float-left align-middle mt-[3px] mr-2">
                    <Icon name="Pin" iconColor="text-body-light" pathName={Pin} />
                  </span>
                  <span class="text-base">12110 Sunset Hills Rd #600 Reston, VA 20190</span>
                </div>
              </li>
              <li class="my-3">
                <a href="tel:+13115552368" class="text-body-light inline-block">
                  <span class="relative float-left align-middle mt-[3px] mr-2">
                    <Icon name="Phone" iconColor="text-body-light" pathName={Phone} />
                  </span>
                  <span class="text-base">(703)-259-1199</span>
                </a>
              </li>
              <li class="my-3">
                <a href="mailto:hello@evidentstack.com" class="text-body-light inline-block">
                  <span class="relative float-left align-middle mt-[3px] mr-2">
                    <Icon name="Email" iconColor="text-body-light" pathName={Email} />
                  </span>
                  <span class="text-base">hello@evidentstack.com</span>
                </a>
              </li>
            </ul>
          </address>
        </Column>
        <Column class="lg:order-last order-first">
          <div class="w-full md:p-4">
            <Form action="create_ticket" method="post" formClass="md:p-4 w-full">
              <ul>
                <li class="mb-4">
                  <Input type="text" placeholder="Subject" name="subject" class="border" required />
                </li>
                <li class="mb-4">
                  <Textarea placeholder="Message" name="description" rows="6" class="" required />
                </li>
                <li class="mb-4">
                  <Input
                    type="email"
                    placeholder="your email address"
                    name="email"
                    class="border"
                    required
                  />
                </li>
                <li class="mb-4">
                  <Submit
                    type="submit"
                    value="Submit"
                    color="evidentsystems"
                    size="md"
                    class="mt-4"
                  />
                </li>
              </ul>
            </Form>
          </div>
        </Column>
      </Row>
    </span>
  </Container>
</main>
