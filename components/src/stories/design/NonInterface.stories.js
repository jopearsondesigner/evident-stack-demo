import NonInterface from '../../lib/design/grid/placement/NonInterface.svelte';

export default {
	title: 'Evident Design/NonInterface Card',
	component: NonInterface,
	argTypes: {
    type: {
      control: { type: 'select' },
      options: ['command', 'event', 'readModel']
    },
    title: {type: 'string'},
    description: {type: 'string'}
  }
};

// eslint-disable-next-line no-unused-vars
const Template = (
  /** @type {} */
  args
) => ({
  Component: NonInterface,
  props: args
});

export const Default = Template.bind({});
Default.args = {
  type: "command",
  title: "Send Command",
  description: "This is a **nice** description"
};

export const Command = Template.bind({});
// @ts-ignore
Command.args = {
  type: "command",
  title: "Send Command"
};

export const Event = Template.bind({});
// @ts-ignore
Event.args = {
  type: "event",
  title: "Event Occurred"
};

export const ReadModel = Template.bind({});
// @ts-ignore
ReadModel.args = {
  type: "readModel",
  title: "Some State"
};
