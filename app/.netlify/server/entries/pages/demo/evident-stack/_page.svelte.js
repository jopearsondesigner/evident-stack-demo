import { c as create_ssr_component, v as validate_component, e as escape } from '../../../../chunks/index3.js';
import { L as Logo, N as Navbar, a as NavInner, b as NavToolbar, c as NavBrand, T as ThemeSwitch } from '../../../../chunks/ArrowDown.js';
import { D as DesignLogo, a as DataLogo, b as DomainFunctionsLogo, c as DeployLogo, d as DatabaseLogo, N as NavHamburger, e as DrawerDetails, A as Accordion, S as SidebarContainer, f as SidebarGroup, g as SidebarItem, T as TreeView, h as TreeItem } from '../../../../chunks/TreeItem.js';
import { D as Drawer, S as Sidebar } from '../../../../chunks/Sidebar.js';
import { F as FlowAnchor, d as default_decider, G as Grid } from '../../../../chunks/Grid.js';
import { B as Button } from '../../../../chunks/Button.js';

const columnCount = 16;
const placement = (kind, id, index, name, description, interfaceKind = "blank") => ({
  kind,
  id,
  component_id: id.replace("placement-", ""),
  index,
  name,
  description,
  interface_config: kind === "interface" ? {
    kind: interfaceKind
  } : void 0
});
const placements = {
  ownerApp: placement(
    "interface",
    "placement-interface-owner-app",
    1,
    "Owner App",
    "Mobile interface used by vehicle owners.",
    "figma"
  ),
  riderApp: placement(
    "interface",
    "placement-interface-rider-app",
    2,
    "Rider App",
    "Mobile interface used by riders.",
    "figma"
  ),
  vehiclePortal: placement(
    "interface",
    "placement-interface-vehicle-portal",
    4,
    "Vehicle Portal",
    "Administrative interface for fleet and vehicle management.",
    "figma"
  ),
  analyticsDashboard: placement(
    "interface",
    "placement-interface-analytics-dashboard",
    5,
    "Analytics Dashboard",
    "Operational analytics and ride metrics.",
    "figma"
  ),
  opsConsole: placement(
    "interface",
    "placement-interface-ops-console",
    6,
    "Operations Console",
    "Internal operations and dispatch tooling.",
    "job"
  ),
  addVehicle: placement(
    "command",
    "placement-command-add-vehicle",
    2,
    "Add Vehicle",
    "Command triggered when an owner adds a vehicle."
  ),
  vehicleAdded: placement(
    "event",
    "placement-event-vehicle-added",
    3,
    "Vehicle Added",
    "Event recorded after a vehicle is added."
  ),
  vehicleProfile: placement(
    "read_model",
    "placement-read-model-vehicle-profile",
    4,
    "Vehicle Profile",
    "Read model showing vehicle details and availability."
  ),
  requestRide: placement(
    "command",
    "placement-command-request-ride",
    6,
    "Request Ride",
    "Command triggered when a rider requests a ride."
  ),
  rideRequested: placement(
    "event",
    "placement-event-ride-requested",
    7,
    "Ride Requested",
    "Event recorded when a ride request is created."
  ),
  rideStatus: placement(
    "read_model",
    "placement-read-model-ride-status",
    8,
    "Ride Status",
    "Read model representing the rider-facing ride state."
  ),
  assignVehicle: placement(
    "command",
    "placement-command-assign-vehicle",
    10,
    "Assign Vehicle",
    "Command assigning an available vehicle to a ride."
  ),
  vehicleAssigned: placement(
    "event",
    "placement-event-vehicle-assigned",
    11,
    "Vehicle Assigned",
    "Event recorded when a vehicle is assigned."
  ),
  completeRide: placement(
    "command",
    "placement-command-complete-ride",
    12,
    "Complete Ride",
    "Command triggered when a ride completes."
  ),
  rideCompleted: placement(
    "event",
    "placement-event-ride-completed",
    13,
    "Ride Completed",
    "Event recorded after a ride is completed."
  ),
  billingProjection: placement(
    "read_model",
    "placement-read-model-billing-projection",
    14,
    "Billing Projection",
    "Aggregated billing and payment state."
  )
};
const makeCell = (kind, row, column, placement2, laneId) => ({
  kind,
  row,
  column,
  placement: placement2,
  audience: kind === "interface" ? laneId : void 0,
  stream: kind === "event" ? laneId : void 0
});
const createCells = (kind, row, count, cellPlacements, laneId) => Array.from(
  { length: count },
  (_, column) => makeCell(kind, row, column, cellPlacements[column], laneId)
);
const defaultAudience = {
  kind: "default_audience",
  row: 0,
  name: "Interfaces",
  cells: createCells(
    "interface",
    0,
    columnCount,
    {
      1: placements.ownerApp,
      2: placements.riderApp,
      4: placements.vehiclePortal,
      5: placements.analyticsDashboard,
      6: placements.opsConsole
    },
    "default"
  )
};
const ownerAudience = {
  kind: "audience",
  id: "audience-owner",
  index: 1,
  row: 1,
  name: "Owner",
  cells: createCells("interface", 1, columnCount, {}, "audience-owner")
};
const riderAudience = {
  kind: "audience",
  id: "audience-rider",
  index: 2,
  row: 2,
  name: "Rider",
  cells: createCells("interface", 2, columnCount, {}, "audience-rider")
};
const timeline = {
  kind: "timeline",
  row: 3,
  name: "Timeline",
  cells: createCells("timeline", 3, columnCount, {
    2: placements.addVehicle,
    4: placements.vehicleProfile,
    6: placements.requestRide,
    8: placements.rideStatus,
    10: placements.assignVehicle,
    12: placements.completeRide,
    14: placements.billingProjection
  })
};
const vehicleStream = {
  kind: "stream",
  id: "stream-vehicle",
  index: 4,
  row: 4,
  name: "Vehicle",
  cells: createCells(
    "event",
    4,
    columnCount,
    {
      3: placements.vehicleAdded,
      11: placements.vehicleAssigned
    },
    "stream-vehicle"
  )
};
const rideStream = {
  kind: "stream",
  id: "stream-ride",
  index: 5,
  row: 5,
  name: "Ride",
  cells: createCells(
    "event",
    5,
    columnCount,
    {
      7: placements.rideRequested,
      13: placements.rideCompleted
    },
    "stream-ride"
  )
};
const defaultStream = {
  kind: "default_stream",
  row: 6,
  name: "Events",
  cells: createCells("event", 6, columnCount, {}, "default")
};
const lanes = [
  defaultAudience,
  ownerAudience,
  riderAudience,
  timeline,
  vehicleStream,
  rideStream,
  defaultStream
];
const allCells = lanes.flatMap((lane) => lane.cells);
const flows = [
  {
    id: "flow-add-vehicle-to-vehicle-added",
    from: {
      kind: "FlowPort",
      placement_id: placements.addVehicle.id,
      anchor: FlowAnchor.Right
    },
    to: {
      kind: "FlowPort",
      placement_id: placements.vehicleAdded.id,
      anchor: FlowAnchor.Left
    }
  },
  {
    id: "flow-request-ride-to-ride-requested",
    from: {
      kind: "FlowPort",
      placement_id: placements.requestRide.id,
      anchor: FlowAnchor.Right
    },
    to: {
      kind: "FlowPort",
      placement_id: placements.rideRequested.id,
      anchor: FlowAnchor.Left
    }
  },
  {
    id: "flow-assign-vehicle-to-vehicle-assigned",
    from: {
      kind: "FlowPort",
      placement_id: placements.assignVehicle.id,
      anchor: FlowAnchor.Right
    },
    to: {
      kind: "FlowPort",
      placement_id: placements.vehicleAssigned.id,
      anchor: FlowAnchor.Left
    }
  },
  {
    id: "flow-complete-ride-to-ride-completed",
    from: {
      kind: "FlowPort",
      placement_id: placements.completeRide.id,
      anchor: FlowAnchor.Right
    },
    to: {
      kind: "FlowPort",
      placement_id: placements.rideCompleted.id,
      anchor: FlowAnchor.Left
    }
  }
];
const mockGrid = {
  id: "demo-grid-autonomo",
  name: "Autonomo Mobile iOS App",
  description: "Portfolio-safe demo of the Evident Stack event modeling interface.",
  column_count: columnCount,
  row_count: lanes.length,
  default_audience: defaultAudience,
  audiences: [ownerAudience, riderAudience],
  timeline,
  streams: [vehicleStream, rideStream],
  default_stream: defaultStream,
  flows,
  cell_by_row_col: (row, col) => allCells.find((cell) => cell.row === row && cell.column === col)
};
const mockDecider = default_decider;

/* src/routes/demo/evident-stack/+page.svelte generated by Svelte v3.59.1 */
const syncStatus = 0;
const demoHref = '/demo/evident-stack';

const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
	let designExpanded;
	let dataExpanded;
	let domainFunctionsExpanded;
	let deployExpanded;
	let dbExpanded;
	let hidden = false;

	let projectDescriptionOpen = false;
	let expandedLeftNavItem = 'design';

	let isActive;

	const tree_data = [
		{
			name: 'Autonomo Mobile iOS App',
			type: 'event-model',
			id: 1,
			children: [
				{
					name: 'Vehicle',
					type: 'read-model',
					id: 2,
					children: [
						{
							name: 'Add Vehicle',
							type: 'command',
							id: 3
						},
						{
							name: 'Vehicle Added',
							type: 'event',
							id: 4
						},
						{
							name: 'Vehicle Profile',
							type: 'read-model',
							id: 5
						},
						{
							name: 'Owner App',
							type: 'interface',
							id: 6
						}
					]
				},
				{
					name: 'Ride',
					type: 'read-model',
					id: 7,
					children: [
						{
							name: 'Request Ride',
							type: 'command',
							id: 8
						},
						{
							name: 'Ride Requested',
							type: 'event',
							id: 9
						},
						{
							name: 'Ride Status',
							type: 'read-model',
							id: 10
						},
						{
							name: 'Rider App',
							type: 'interface',
							id: 11
						}
					]
				}
			]
		}
	];

	let $$settled;
	let $$rendered;

	do {
		$$settled = true;
		designExpanded = expandedLeftNavItem === 'design';
		dataExpanded = expandedLeftNavItem === 'data';
		domainFunctionsExpanded = expandedLeftNavItem === 'domain-functions';
		deployExpanded = expandedLeftNavItem === 'deploy';
		dbExpanded = expandedLeftNavItem === 'db';

		$$rendered = `${($$result.head += '<!-- HEAD_svelte-j9dkij_START -->' + `${($$result.title = `<title>Evident Stack Demo | Portfolio</title>`, "")}` + '<!-- HEAD_svelte-j9dkij_END -->', "")}

<div class="min-h-screen">${validate_component(Navbar, "Navbar").$$render($$result, { website: false }, {}, {
			default: () => {
				return `${validate_component(NavInner, "NavInner").$$render(
					$$result,
					{
						navDivClass: "flex justify-between items-center"
					},
					{},
					{
						default: () => {
							return `${validate_component(NavToolbar, "NavToolbar").$$render(
								$$result,
								{
									navClass: "px-3 mx-3 h-9 inline-flex space-x-4 items-center"
								},
								{},
								{
									default: () => {
										return `${validate_component(NavHamburger, "NavHamburger").$$render($$result, { website: false, hamburgerClass: "mx-2" }, {}, {})}

        ${validate_component(NavBrand, "NavBrand").$$render(
											$$result,
											{
												src: Logo,
												height: 28,
												logoClass: "flex no-underline mx-3 cursor-default"
											},
											{},
											{}
										)}`;
									}
								}
							)}`;
						}
					}
				)}`;
			}
		})}

  <span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16">${validate_component(ThemeSwitch, "ThemeSwitch").$$render($$result, {}, {}, {})}</span>

  ${validate_component(Drawer, "Drawer").$$render(
			$$result,
			{ placement: "left", hidden },
			{
				hidden: $$value => {
					hidden = $$value;
					$$settled = false;
				}
			},
			{
				default: () => {
					return `${validate_component(Sidebar, "Sidebar").$$render($$result, { class: "w-[240px]" }, {}, {
						default: () => {
							return `${validate_component(DrawerDetails, "DrawerDetails").$$render(
								$$result,
								{
									name: "Autonomo Mobile iOS App",
									description: "Portfolio-safe demo of the Evident Stack event modeling interface.",
									sync_status: syncStatus,
									isOpen: projectDescriptionOpen
								},
								{
									isOpen: $$value => {
										projectDescriptionOpen = $$value;
										$$settled = false;
									}
								},
								{}
							)}

      ${validate_component(Accordion, "Accordion").$$render($$result, {}, {}, {
								default: () => {
									return `${validate_component(SidebarContainer, "SidebarContainer").$$render(
										$$result,
										{
											src: DesignLogo,
											href: demoHref,
											id: "design",
											title: "Design",
											expanded: designExpanded
										},
										{
											expanded: $$value => {
												designExpanded = $$value;
												$$settled = false;
											}
										},
										{
											default: () => {
												return `${validate_component(SidebarGroup, "SidebarGroup").$$render($$result, {}, {}, {
													default: () => {
														return `${validate_component(SidebarItem, "SidebarItem").$$render($$result, { maxHeightNum: 194, blank: true }, {}, {
															default: () => {
																return `${validate_component(Button, "Button").$$render(
																	$$result,
																	{
																		label: "Export JSON",
																		gradient: true,
																		color: "ghost",
																		size: "sm",
																		className: "my-4"
																	},
																	{},
																	{}
																)}`;
															}
														})}`;
													}
												})}`;
											}
										}
									)}

        ${validate_component(SidebarContainer, "SidebarContainer").$$render(
										$$result,
										{
											src: DataLogo,
											href: demoHref,
											id: "data",
											title: "Data",
											expanded: dataExpanded
										},
										{
											expanded: $$value => {
												dataExpanded = $$value;
												$$settled = false;
											}
										},
										{
											default: () => {
												return `${validate_component(SidebarGroup, "SidebarGroup").$$render($$result, {}, {}, {
													default: () => {
														return `${validate_component(SidebarItem, "SidebarItem").$$render(
															$$result,
															{
																padding: "p-0",
																maxHeightNum: 322,
																blank: true
															},
															{},
															{
																default: () => {
																	return `${validate_component(TreeView, "TreeView").$$render(
																		$$result,
																		{ tree_data, isActive },
																		{
																			isActive: $$value => {
																				isActive = $$value;
																				$$settled = false;
																			}
																		},
																		{
																			default: ({ item }) => {
																				return `${item.children
																				? `${validate_component(TreeItem, "TreeItem").$$render(
																						$$result,
																						{
																							elementClass: "flex items-center w-full group h-7",
																							href: `#tree-${item.id}`
																						},
																						{},
																						{
																							default: () => {
																								return `<span class="ml-1 text-xs font-semibold">${escape(item.name)}</span>`;
																							}
																						}
																					)}`
																				: `${validate_component(TreeItem, "TreeItem").$$render(
																						$$result,
																						{
																							elementClass: "pl-[35px] flex items-center w-full h-7 text-body dark:text-body-dark text-default bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20]",
																							href: `#tree-${item.id}`
																						},
																						{},
																						{
																							default: () => {
																								return `<span class="ml-1">${escape(item.name)}</span>`;
																							}
																						}
																					)}`}`;
																			}
																		}
																	)}`;
																}
															}
														)}`;
													}
												})}`;
											}
										}
									)}

        ${validate_component(SidebarContainer, "SidebarContainer").$$render(
										$$result,
										{
											src: DomainFunctionsLogo,
											href: demoHref,
											id: "domain-functions",
											title: "Domain Functions",
											expanded: domainFunctionsExpanded
										},
										{
											expanded: $$value => {
												domainFunctionsExpanded = $$value;
												$$settled = false;
											}
										},
										{
											default: () => {
												return `${validate_component(SidebarGroup, "SidebarGroup").$$render($$result, {}, {}, {})}`;
											}
										}
									)}

        ${validate_component(SidebarContainer, "SidebarContainer").$$render(
										$$result,
										{
											src: DeployLogo,
											href: demoHref,
											id: "deploy",
											title: "Deploy",
											expanded: deployExpanded
										},
										{
											expanded: $$value => {
												deployExpanded = $$value;
												$$settled = false;
											}
										},
										{
											default: () => {
												return `${validate_component(SidebarGroup, "SidebarGroup").$$render($$result, {}, {}, {})}`;
											}
										}
									)}

        ${validate_component(SidebarContainer, "SidebarContainer").$$render(
										$$result,
										{
											src: DatabaseLogo,
											href: demoHref,
											id: "db",
											title: "Database",
											expanded: dbExpanded
										},
										{
											expanded: $$value => {
												dbExpanded = $$value;
												$$settled = false;
											}
										},
										{
											default: () => {
												return `${validate_component(SidebarGroup, "SidebarGroup").$$render($$result, {}, {}, {})}`;
											}
										}
									)}`;
								}
							})}`;
						}
					})}`;
				}
			}
		)}

  <main class="${[
			"relative left-0 right-0 transition-all duration-[200ms] pt-16 ml-0 h-screen overflow-hidden",
			!hidden ? "ml-[240px]" : ""
		].join(' ').trim()}"><div class="absolute top-20 left-6 z-30 max-w-md rounded-md bg-white/90 dark:bg-dark-2/90 backdrop-blur-sm border border-gray-secondary dark:border-border-dark shadow-header px-4 py-3"><p class="text-xs uppercase tracking-wide text-gray-brand-2 dark:text-gray-brand-4">Portfolio Demo
      </p>

      <h1 class="mt-1 text-sm font-bold text-body-light dark:text-body-dark">Evident Stack SaaS Interface
      </h1>

      <p class="mt-1 text-xs leading-normal text-body-light dark:text-gray-brand-4">A portfolio-safe demo of real product UI, design systems, event modeling, and front-end
        implementation work.
      </p></div>
    ${validate_component(Grid, "Grid").$$render(
			$$result,
			{
				mode: "navigation",
				grid: mockGrid,
				decider: mockDecider
			},
			{},
			{}
		)}</main></div>`;
	} while (!$$settled);

	return $$rendered;
});

export { Page as default };
