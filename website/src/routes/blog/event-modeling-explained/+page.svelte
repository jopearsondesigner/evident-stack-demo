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
	import FeaturedBlogImage from '$components/assets/images/blog/blog-photo-1.png';
	import A from '$components/typography/A.svelte';
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
						Event Modeling Explained
					</SectionHeader>
					<SectionCopy classes="ml-4" pClass="self-start text-gray-primary flex">
						&nbsp;
						<span>By</span>
						&nbsp;
						<A href="">Marketing</A>
						&nbsp;
						<span>&bull;</span>
						&nbsp;
						<time>August 24, 2020</time>
					</SectionCopy>
				</Section>
			</Column>
			<!-- Column -->
		</Row>
		<!-- Row -->
	</ArticleHead>
	<ArticleBody>
		<Row class="sm:container mx-auto px-4 grid lg:grid-cols-6 py-16">
			<Column class="lg:col-start-2 col-span-4">
				<Section name="blog">
					<SectionCopy classes="mb-4">
						It’s no secret that tech developers and senior executives don’t understand each other.
						Execs tend to have lofty ideas for the perfect customer experience. Developers are
						constrained to what the technology is actually capable of. Luckily, product managers
						came along to try and mediate the developer-executive relationship. But even still,
						miscommunication and frustration seem to dominate software development these days.
					</SectionCopy>
					<SectionCopy classes="mb-4">
						That’s why <A
							class="transition duration-300 ease-in"
							href="https://www.linkedin.com/in/eventmodeling/">Adam Dymitruk</A
						>, in a stroke of genius, dreamt up <A
							class="transition duration-300 ease-in"
							href="https://eventmodeling.org/">Event Modeling</A
						>. Adam makes the point that story-telling is the innate human way to convey
						information. <A
							class="transition duration-300 ease-in"
							href="https://eventmodeling.org/posts/what-is-event-modeling/"
							>“Our brains are built for [story-telling] more than they are built for flow-charts
							and other formats”</A
						>. Event Modeling harnesses the commonalities in the human thought process and memory
						storage. It creates a shared language–a language that developers, senior executives,
						product managers, and designers all understand.
					</SectionCopy>
					<SectionHeader tag="h4" customSize="text-[24px]">
						Event Modeling is a way to graphically represent change in a system over time.
					</SectionHeader>
					<SectionCopy classes="mb-4">
						A simple way to understand Event Modeling is with an example of a customer checking into
						a hotel. There are four components to the process:
					</SectionCopy>
					<SectionCopy classes="mb-4">
						First, you and your team brainstorm the fundamental steps in a user’s journey. Next,
						place them on a simple, graphical timeline. You tell the story of the user’s journey.
						These steps are called events. It’s important to keep in mind that events are specific
						occurrences that change the state of the process. In our hotel example, a customer uses
						an app to check-in to a hotel, leaving one less room available to other customers.
						Events are represented by orange sticky notes.
					</SectionCopy>
					<SectionCopy classes="mb-4">
						Events can only change the state of the process after the user gives a command. In other
						words, the command precedes the event. In a hotel reservation interface, the command
						would be the user selecting “check-in.” The event is a result of that command:
						“checked-in.” Commands are represented by blue sticky notes.
					</SectionCopy>
					<SectionCopy classes="mb-4">
						Next, the read-model. Read-models are the result of the command-event chain. It’s what
						the user can see. The user selected “check-in”, then they were “checked-in.” The
						read-model is a calendar that displays room availability for certain dates. Read-models
						are represented by green sticky notes.
					</SectionCopy>
					<SectionCopy classes="mb-4">
						Interfaces (or wireframes) are simple mock-ups of what the interface could look like.
						This helps the visual learners in the group. It also gives non-designers a chance to
						give their input on what would make a nice user experience. Pretty simple, right?
					</SectionCopy>
					<SectionHeader tag="h4" customSize="text-[24px]">
						It’s time to improve your company’s software development process.
					</SectionHeader>
					<SectionCopy classes="mb-4">
						The software development world was ready for a big change. No more miscommunications
						from developer → product manager → designer → executive. Event Modeling simplifies the
						communication process. Executives can understand what’s actually possible. The developer
						can understand exactly what the executives want to deliver to their customers. The
						designer can make it all look beautiful. And, no one shoots the poor product manager.
					</SectionCopy>
					<SectionCopy classes="mb-4">
						That’s Event Modeling in a nutshell. If you’re still curious about the ins and outs of
						Event Modeling, check out the rest of <A
							class="transition duration-300 ease-in"
							href="https://eventmodeling.org/posts/what-is-event-modeling/"
							>Adam Dymitruk’s technical writing piece</A
						> that’s featured on his blog.
					</SectionCopy>
					<SectionCopy classes="mb-4">
						As for why you need oNote? That’s a whole long list of things that <A
							class="transition duration-300 ease-in"
							href="https://www.onote.com/contact/">we’d love to talk to you about</A
						>.
					</SectionCopy>
				</Section>
			</Column>
		</Row>
	</ArticleBody>
</Article>

<A class="transition duration-300 ease-in" href="" />
