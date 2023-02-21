<script lang="ts">
	import Article from '$components/blog/Article.svelte';
	import ArticleHead from '$components/blog/ArticleHead.svelte';
	import ArticleBody from '$components/blog/ArticleBody.svelte';
	import Row from '$components/Row.svelte';
	import Column from '$components/Row.svelte';
	import Section from '$components/Section.svelte';
	import SectionHeader from '$components/typography/SectionHeader.svelte';
	import SectionCopy from '$components/typography/SectionCopy.svelte';
	import YouTube from '$components/utils/YouTube.svelte';
	let WebinarVideo = 'https://www.youtube.com/embed/i_uPgCD9co8?feature=oembed';
</script>

<Article>
	<ArticleHead>
		<Row class="sm:container px-4 grid lg:grid-cols-6 px-4 py-16">
			<Column class="lg:col-start-2 col-span-4">
				<Section name="blog">
					<SectionHeader
						tag="h1"
						class="text-body-2 dark:text-body-4"
						color="text-gray-brand-2"
						customSize="lg:text-[30px] text-[24px]"
					>
						Accelerate from Event Model to Implementation with oNote Code-generation & Event Store
					</SectionHeader>
					<YouTube src={WebinarVideo} />
				</Section>
			</Column>
		</Row>
	</ArticleHead>
	<ArticleBody>
		<Row class="sm:container mx-auto px-4 grid lg:grid-cols-6 pb-16">
			<Column class="lg:col-start-2 col-span-4">
				<Section name="blog">
					<SectionCopy classes="my-4">
						Learn from some of the leading experts in software design and software development.
					</SectionCopy>
					<SectionCopy classes="my-4">
						Bobby Calderwood the founder of Evident system and inventor of oNote, and Chris Condron
						the CTO of Event Store are here to train on:
						<ul class="list-disc m-4">
							<li>
								Building a simple and business valuable Event Model, defining data schemas for
								events
							</li>
							<li>Building request and response schemas for commands and read models</li>
							<li>Generating Avro Event schemas, and uploading to Event StoreDB Schema Registry</li>
							<li>
								Generating Avro Protocol for commands and Read Models, and generating a gRPC
								implementation Scaffold
							</li>
							<li>Implementing business logic with your companies event streaming platform</li>
						</ul>
					</SectionCopy>
				</Section>
			</Column>
		</Row>
	</ArticleBody>
</Article>
