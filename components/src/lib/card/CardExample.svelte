<script lang='ts'>
    import Card from './Card.svelte';
    import Container from '../Container.svelte';
    import Row from '../Row.svelte';
    import Column from '../Column.svelte';
    import Cash from '../assets/images/team/cash.png';
</script>

<Container class='container my-16'>
    <Row class='grid lg:grid-cols-3 md:grid-cols-2 grid-cols-1 gap-4'>
        <Column>
            <Card img={Cash} size='sm'>

            </Card>
        </Column>
    </Row>
</Container>

