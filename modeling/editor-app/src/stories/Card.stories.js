import Card from '../lib/components/Card.svelte';

export default {
  title: 'Event Model/Card',
  component: Card,
  argTypes: {
    backgroundColor: { control: 'color' },
    label: { control: 'text' },
    type_: {
      control: { type: 'select' },
      options: ['interface', 'command', 'event', 'readModel'],
    },
  },
};

// More on component templates: https://storybook.js.org/docs/svelte/writing-stories/introduction#using-args
const Template = (args) => ({
  Component: Card,
  props: args,
});

export const Interface = Template.bind({});
Interface.args = {
  type_: 'interface',
  label: 'A simple interface',
};

export const Command = Template.bind({});
Command.args = {
  type_: 'command',
  label: 'A simple command',
};

export const Event = Template.bind({});
Event.args = {
  type_: 'event',
  label: 'A simple event',
};

export const ReadModel = Template.bind({});
ReadModel.args = {
  type_: 'readModel',
  label: 'A simple read model',
};
