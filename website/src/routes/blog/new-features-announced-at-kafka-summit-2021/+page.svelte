<script lang="ts">
	import classNames from 'classnames';
	import Article from '$components/blog/Article.svelte';
	import ArticleHead from '$components/blog/ArticleHead.svelte';
	import ArticleBody from '$components/blog/ArticleBody.svelte';
	import Row from '$components/Row.svelte';
	import Column from '$components/Row.svelte';
	import Section from '$components/Section.svelte';
	import SectionHeader from '$components/typography/SectionHeader.svelte';
	import SectionCopy from '$components/typography/SectionCopy.svelte';
	import FeaturedBlogImage from '$components/assets/images/blog/blog-photo-4.png';
	import A from '$components/typography/A.svelte';
	import BlogImg1 from '$components/assets/images/blog/ss-navigation.png';
	import BlogImg2 from '$components/assets/images/blog/ss-schemas.png';
	import Button from '$components/Button.svelte';
	let customSpan: string = 'lg:col-span-3';
</script>

<Article>
	<ArticleHead
		divClass="relative overflow-hidden min-h-[400px] grid grid-cols-6 bg-origin-border bg-center bg-no-repeat bg-cover"
		style="background-image: url({FeaturedBlogImage})"
	>
		<Row class="grid grid-cols-6 col-span-3 h-auto relative left-0 right-0 bg-black bg-opacity-80">
			<Column class={classNames('lg:col-start-3 col-start-1 col-end-7', customSpan)}>
				<Section
					name="blog"
					sectionClass="z-10 h-full flex flex-col justify-center items-end px-4 py-4"
				>
					<SectionHeader
						tag="h1"
						class="lg:pl-4 lg:text-left text-center"
						color="text-gray-primary"
						customSize="2xl:text-[56px] xl:text-[56px] lg:text-[50px] md:text-[50px] text-[30px]"
					>
						New Features Announced at Kafka Summit 2021
					</SectionHeader>
					<SectionCopy classes="ml-4" pClass="self-start text-gray-primary flex">
						&nbsp;
						<span>By</span>
						&nbsp;
						<A href="">Terry Blessing</A>
						&nbsp;
						<span>&bull;</span>
						&nbsp;
						<time>October 14, 2021</time>
					</SectionCopy>
				</Section>
			</Column>
			<!-- Column -->
		</Row>
		<!-- Row -->
	</ArticleHead>
	<ArticleBody>
		<Row class="sm:container px-4 grid lg:grid-cols-6 py-16">
			<Column class="lg:col-start-2 col-span-4">
				<Section name="blog">
					<SectionCopy classes="mb-4">
						The oNote team just got back from presenting at Kafka Summit Americas 2021, where we
						announced lots of exciting new features that put event modeling at the heart of
						event-driven system design and implementation! Let’s review all of these enhanced and
						new features now available in oNote.
					</SectionCopy>
					<SectionHeader tag="h2" customSize="text-[32px]">
						Totally redesigned event modeling user interface
					</SectionHeader>
					<SectionCopy classes="mb-4">
						The user interface of oNote has been redesigned with speed, convenience, and the user
						experience in mind. The revamped design takes the guesswork out of which model objects
						and actions are allowed in which lanes. Built-in guardrails guide you to design valid
						models of your event-driven applications. These new features accelerate your design and
						implementation processes for a successful event-driven ecosystem.
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">Flexible canvas workspace</SectionHeader>
					<SectionCopy classes="mb-4">
						Navigate and extend the canvas as you prefer:
						<ul class="list-disc m-4">
							<li>
								Left main navigation menu to access features and models. Collapse or expand the menu
								bar as needed to maximize your canvas workspace.
							</li>
							<li>
								Convenient <A
									class="transition duration-300 ease-in"
									href="https://docs.onote.com/onote-docs/Latest/keyboard-shortcuts.html"
									>keyboard shortcuts</A
								> to navigate within your models. Keys and key combinations are supported for Vim, Emacs,
								and standard keyboard mappings. Anywhere on the model canvas, press Shift + ? to access
								an online help page for keyboard shortcuts. Of course, you can always right-click to
								show the context menu.
							</li>
							<li>
								<A
									class="transition duration-300 ease-in"
									href="https://docs.onote.com/onote-docs/Latest/lanes/add-lane.html"
									>Insert lanes</A
								> and <A
									class="transition duration-300 ease-in"
									href="https://docs.onote.com/onote-docs/Latest/columns/insert-column.html"
									>columns</A
								> as needed to expand your model dimensions.
							</li>
							<li>
								<A
									class="https://docs.onote.com/onote-docs/Latest/models/import-export-models.html#export-event-model-json"
									href="">Export models</A
								> for reuse, collaboration, and safekeeping.
							</li>
							<li>
								<A
									class="transition duration-300 ease-in"
									href="https://docs.test.onote.com/onote-docs/Latest/links/delete-link.html"
									>Delete a flow</A
								> (linked arrow) between objects.
							</li>
						</ul>
						For more details, read the UI Tour topic and view its YouTube videos embedded in the docs.
						<img src={BlogImg1} alt="Navigation" class="my-16" />
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">Improved model imports</SectionHeader>
					<SectionCopy classes="my-4">
						<strong>Vastly improved the model import feature!</strong> Import entire models and model
						snippets at a pinpointed cursor location within a model. Because the import now shifts any
						existing populated grid cells to the right, there are no longer any concerns about overwriting
						your models when importing. This new behavior facilitates inserting model snippets and reuse
						of existing models. oNote is working on a repository of models and snippets you can import
						into your models.
					</SectionCopy>
					<SectionCopy classes="my-4">
						For details, see <A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/models/import-export-models.html"
							>importing a model</A
						>.
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">Cut, copy, and paste</SectionHeader>
					<SectionCopy classes="my-4">
						Cutting, copying, and pasting model objects are now supported. Use the <A
							class="transition duration-300 ease-in"
							href="https://docs.test.onote.com/onote-docs/Latest/keyboard-shortcuts.html#_cut_or_copy_and_paste_a_model_object"
							>keyboard shortcuts</A
						> or the <A
							class="transition duration-300 ease-in"
							href="https://docs.test.onote.com/onote-docs/Latest/ui-tour.html#rclick-menus"
							>context menu</A
						> options to conveniently cut, copy, and paste your model objects.
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">Smart cursor mode</SectionHeader>
					<SectionCopy classes="my-4">
						If you are new to event modeling, or want to quickly create the basic foundation of a
						model, you can use the smart cursor mode. Smart cursor is a predictive auto-completion
						feature. For a demo, view the smart <A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/ui-tour.html#smart-cursor"
							>cursor mode videos></A
						>.
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">Brainstorm mode</SectionHeader>
					<SectionCopy classes="my-4">
						If you are new to event modeling, or want to quickly create the basic foundation of a
						model, you can use the smart cursor mode. Smart cursor is a predictive auto-completion
						feature. For a demo, view the smart <A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/ui-tour.html#smart-cursor"
							>cursor mode videos</A
						>.
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">Interface designer</SectionHeader>
					<SectionCopy classes="my-4">
						The Interface designer has a separate side canvas that makes it convenient for you to
						model interface objects such as automated jobs, define endpoints for your <A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/interfaces/rest-interface.html"
							>RESTful interfaces</A
						>, <A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/interfaces/html-interface.html"
							>HTML elements</A
						> if you aren’t using Figma, a temporary <A
							class="transition duration-300 ease-in"
							href="">blank</A
						> placeholder, or a URL to a <A class="transition duration-300 ease-in" href=""
							>Figma wireframe</A
						>. Draw data flow arrows directly to and from preliminary interfaces without having to
						add interface elements first.
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">Figma embed interfaces</SectionHeader>
					<SectionCopy classes="my-4">
						Visualize high-fidelity UX designs with <A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/interfaces/figma-interface.html"
							>Figma embed interfaces</A
						>. Display Figma wireframes directly within your event model. This feature requires a
						paid subscription plan.
					</SectionCopy>
					<SectionHeader tag="h2" customSize="text-[32px]">Define schemas</SectionHeader>
					<SectionCopy classes="my-4">
						<A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/schemas/schemas-overview.html"
							>Define schemas</A
						> to export from oNote and manage within the <A
							class="transition duration-300 ease-in"
							href="https://docs.confluent.io/platform/current/schema-registry/index.html"
							>Confluent Schema Registry</A
						>. Validate a sample user JSON object against your schema right within the Schema panel
						in oNote.
					</SectionCopy>
					<SectionCopy classes="my-4">
						Evolve your models and schemas in tandem with your business requirements. With
						event-driven systems like Apache Kafka and Confluent Cloud/Platform, schema management
						is a must.
						<img src={BlogImg2} alt="Schemas" class="my-16" />
					</SectionCopy>
					<SectionHeader tag="h2" customSize="text-[32px]">Generate Avro code</SectionHeader>
					<SectionCopy classes="my-4">
						<A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/code/generate-avro.html"
							>Generate Avro code</A
						> that exports an Avro protocol (.avpr) file that developers can use to accelerate development
						of a gRPC client or server that implements the web service contract defined by your Commands
						and Read Models. This export also includes the set of Event schema (.avsc) files organized
						by Stream, for example to upload to the <A
							class="transition duration-300 ease-in"
							href="https://docs.confluent.io/platform/current/schema-registry/index.html"
							>Confluent Schema Registry</A
						>. The Avro ecosystem includes robust tools for generating runtime code (e.g. Java, C#,
						Python, etc.) from the exported protocol and schema files.
					</SectionCopy>
					<SectionHeader tag="h2" customSize="text-[32px]">Redesigned website</SectionHeader>
					<SectionCopy classes="my-4">
						The oNote website has been redesigned and the content updated. <A
							class="transition duration-300 ease-in"
							href="https://www.onote.com/">Take a look</A
						>!
					</SectionCopy>
					<SectionHeader tag="h2" customSize="text-[32px]">New doc site</SectionHeader>
					<SectionCopy classes="my-4">
						<A
							class="transition duration-300 ease-in"
							href="https://docs.onote.com/onote-docs/Latest/index.html">The documentation site</A
						> for oNote is now available. Get the information you need to learn about event modeling
						with oNote. Click the book icon in the upper-right corner of the oNote app to access the
						documentation. There are browse sequences so you can peruse the entire doc set. Breadcrumbs
						orient you to your current location in the docs and also provide another way to navigate.
						A table of contents is available for the entire doc set (left TOC) and within each topic
						with multiple sections (right TOC). More videos, tutorials, and a search feature are in the
						works.
					</SectionCopy>
					<SectionHeader tag="h2" customSize="text-[32px]">
						Flexible subscription plans that grow with your business
					</SectionHeader>
					<SectionCopy classes="my-4">
						Choose from oNote Starter (free), Essential, and Premium subscription plans. If you are
						on the Essential subscription, add only the PowerUp features that you need. Upvote the
						upcoming PowerUp features to expedite development of those most critical to your needs.
						For more details, check out the Profile <A
							class="transition duration-300 ease-in"
							href="">Subscriptions</A
						> page.
					</SectionCopy>
					<SectionHeader tag="h3" customSize="text-[24px]">oNote features roadmap</SectionHeader>
					<SectionCopy classes="my-4">
						There is much more to come with planned future features and those PowerUps as noted on
						the Profile Subscriptions page. Stay tuned!
					</SectionCopy>
					<SectionHeader tag="h2" customSize="text-[32px]">
						Sign up for oNote and upgrade to Essential
					</SectionHeader>
					<SectionCopy classes="my-4">
						If you haven’t already, <A
							class="transition duration-300 ease-in"
							href="https://app.onote.com/sign-up">sign up</A
						> for oNote and <A
							class="transition duration-300 ease-in"
							href="https://app.onote.com/profile/billing/subscription?add=price_1JZrzbDAZ6jOceePJ3svOFp3"
							>upgrade to the Essential plan</A
						> to try out all the new features and gain access to our PowerUps. Of course, our free tier
						oNote Starter is available for you to try out our basic event modeling features.
					</SectionCopy>
					<Column>
						<Button
							gradient
							color="brandDesignPrimary"
							size="sm"
							label="Start Using Schemas!"
							class="mt-8 block m-auto"
							href="https://app.onote.com/"
						/>
					</Column>
				</Section>
			</Column>
		</Row>
	</ArticleBody>
</Article>
