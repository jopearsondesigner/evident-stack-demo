// TODO: Snapshot event prunes any component definitions not present in placements
//  In a convergent context, we can't know when deleting a placement whether it's
//  the last placement for a given component definition in order to delete that
//  definition.  So we should leave the definitions in place until a snapshot,
//  then prune. Subsequent additions of placements against that definition
//  should then fail
mod text;
mod event_model;
