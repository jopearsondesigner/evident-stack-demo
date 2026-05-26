(function (authHelpersSveltekit, dexie, base64Js) {
    'use strict';

    (async ()=>{
        onmessage = async (e)=>{
            const { debug  } = await Promise.resolve().then(function() {
                return util;
            });
            const { initializeDexie , connect  } = await Promise.resolve().then(function() {
                return dexie$1;
            });
            await import('dexie-observable').then(async (m)=>{
                await m.__tla;
                return m;
            });
            await import('dexie-syncable').then(async (m)=>{
                await m.__tla;
                return m;
            });
            initializeDexie();
            debug("Worker received message", e);
            let { url , session  } = e.data;
            if (url && session) {
                debug("Worker connecting to sync service", url, session);
                try {
                    await connect(url, session, (status)=>{
                        debug("New connection status on worker:", status);
                        postMessage({
                            status
                        });
                    });
                } catch (e2) {
                    console.error("Error while connecting sync worker", e2);
                }
                debug("Worker connected to sync service", url);
            }
        };
        const debug = (...args)=>{};
        var util = Object.freeze({
            __proto__: null,
            debug: debug
        });
        const PUBLIC_SUPABASE_URL = "http://localhost:54321";
        const PUBLIC_SUPABASE_ANON_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImV4cCI6MTk4MzgxMjk5Nn0.EGIM96RAZx35lJzdJsyH-qQwv8Hdp7fsn3W0YpN81IU";
        let supabase;
        const init_supabase = (session, f)=>{
            supabase = authHelpersSveltekit.createSupabaseLoadClient({
                supabaseUrl: PUBLIC_SUPABASE_URL,
                supabaseKey: PUBLIC_SUPABASE_ANON_KEY,
                event: {
                    fetch: f ? f : fetch
                },
                serverSession: session
            });
            return supabase;
        };
        const CREATE = 1;
        const UPDATE = 2;
        const DELETE = 3;
        const INITIAL_BACKOFF = 2;
        let local_to_remote_backoff = INITIAL_BACKOFF;
        let remote_to_local_backoff = INITIAL_BACKOFF;
        const dexie_change_to_model_obj = (change)=>{
            let obj = change.obj;
            return {
                id: obj.id,
                name: obj.name,
                description: obj.description
            };
        };
        const dexie_change_to_patch_obj = (change)=>{
            let obj = change.obj;
            return {
                id: obj.id,
                model: obj.model,
                user: obj.user,
                data: base64Js.fromByteArray(obj.data)
            };
        };
        const SupabaseSync = {
            sync: async function(context, url, options, baseRevision, syncedRevision, changes, partial, applyRemoteChanges, onChangesAccepted, onSuccess, onError) {
                const session = await supabase.auth.getSession();
                if (!session.data.session) {
                    onError(`User is not logged in: ${session.error}`);
                    return;
                }
                const user_id = session.data.session.user.id;
                const send_changes = async (new_changes, _base_revision, _is_partial, on_changes_accepted)=>{
                    let changes2 = new_changes.reduce((acc, change)=>{
                        switch(change.type){
                            case CREATE:
                                if (change.table == options.local_models_table) {
                                    acc.model_insertions.push(dexie_change_to_model_obj(change));
                                } else if (change.table == options.local_patches_table) {
                                    acc.patch_insertions.push(dexie_change_to_patch_obj(change));
                                }
                                break;
                            case UPDATE:
                                if (change.table == options.local_models_table) {
                                    acc.model_updates.push(dexie_change_to_model_obj(change));
                                }
                                break;
                            case DELETE:
                                if (change.table == options.local_models_table) {
                                    acc.model_deletions.push(change.key);
                                }
                                break;
                        }
                        return acc;
                    }, {
                        patch_insertions: [],
                        model_insertions: [],
                        model_updates: [],
                        model_deletions: []
                    });
                    let { error: error2 , ...response2 } = await supabase.rpc("apply_client_changes", {
                        changes: changes2
                    });
                    debug("apply_client_changes response:", {
                        error: error2,
                        ...response2
                    });
                    if (error2) {
                        console.warn("Error applying local changes to remote server:", error2);
                        local_to_remote_backoff *= 2;
                        onError(error2, local_to_remote_backoff);
                    } else {
                        local_to_remote_backoff = INITIAL_BACKOFF;
                        on_changes_accepted();
                    }
                };
                const model_event_to_dexie_change = (event)=>{
                    switch(event.type){
                        case "created":
                            return [
                                {
                                    type: CREATE,
                                    table: options.local_models_table,
                                    key: event.subject,
                                    obj: {
                                        ...event.data,
                                        user: user_id
                                    }
                                }
                            ];
                        case "updated":
                            return [
                                {
                                    type: UPDATE,
                                    table: options.local_models_table,
                                    key: event.subject,
                                    mods: {
                                        name: event.data.name,
                                        description: event.data.description
                                    },
                                    obj: {
                                        name: event.data.name,
                                        description: event.data.description
                                    },
                                    oldObj: null
                                }
                            ];
                        case "deleted":
                            return [
                                {
                                    type: DELETE,
                                    table: options.local_models_table,
                                    key: event.subject,
                                    oldObj: null
                                }
                            ];
                        case "patched":
                            return [
                                {
                                    type: CREATE,
                                    table: options.local_patches_table,
                                    key: event.subject,
                                    obj: {
                                        id: event.data.patch_id,
                                        model: event.subject,
                                        data: base64Js.toByteArray(event.patch_data),
                                        user: event.data.user
                                    }
                                }
                            ];
                        case "collaborator_role_granted":
                            return [];
                        case "collaborator_role_revoked":
                            return [
                                {
                                    type: DELETE,
                                    table: options.local_models_table,
                                    key: event.subject,
                                    oldObj: null
                                }
                            ];
                        case "snapshotted":
                            return [
                                {
                                    type: CREATE,
                                    table: options.local_patches_table,
                                    key: event.subject,
                                    obj: {
                                        id: event.data.patch_id,
                                        model: event.subject,
                                        data: base64Js.toByteArray(event.patch_data)
                                    }
                                },
                                ...event.data.obsolete_patch_ids.map((id)=>{
                                    return {
                                        type: DELETE,
                                        table: options.local_patches_table,
                                        key: id,
                                        oldObj: null
                                    };
                                })
                            ];
                    }
                    return [];
                };
                const channel = supabase.channel("evidentsystems-model-events");
                channel.on("postgres_changes", {
                    event: "INSERT",
                    schema: options.remote_schema
                }, async (payload)=>{
                    if (payload.table == options.remote_patches_table) {
                        let patch = payload["new"];
                        context.unapplied_patches = {
                            ...context.unapplied_patches,
                            [patch.id]: patch
                        };
                        await context.save();
                    } else if (payload.table == options.remote_events_table) {
                        let event = payload["new"];
                        if (event.type == "patched" || event.type == "snapshotted") {
                            let patch = context.unapplied_patches[event.data.patch_id];
                            delete context.unapplied_patches[event.data.patch_id];
                            event.patch_data = patch.data;
                            await applyRemoteChanges(model_event_to_dexie_change(event), event.id);
                            await context.save();
                        } else {
                            await applyRemoteChanges(model_event_to_dexie_change(event), event.id);
                        }
                    }
                }).subscribe();
                const cleanup = ()=>{
                    supabase.removeChannel(channel);
                };
                let { error , data , ...response } = await supabase.rpc("model_events_since", {
                    starting_event_id: syncedRevision
                });
                debug("Received events: since event id:", syncedRevision, "response:", {
                    error,
                    data,
                    ...response
                });
                if (error) {
                    console.warn("Error fetching remote changes from remote server:", error);
                    remote_to_local_backoff *= 2;
                    onError(error, remote_to_local_backoff);
                } else {
                    remote_to_local_backoff = INITIAL_BACKOFF;
                    const changes2 = data?.flatMap(model_event_to_dexie_change) || [];
                    const revision = data?.length && data.length > 0 ? data[data.length - 1].id : null;
                    await applyRemoteChanges(changes2, revision);
                    onSuccess({
                        react: send_changes,
                        disconnect: cleanup
                    });
                }
                send_changes(changes, baseRevision, partial, onChangesAccepted);
            }
        };
        class EventModelDatabase extends dexie.Dexie {
            model_patches;
            models;
            constructor(){
                super("evidentstack");
                this.version(1).stores({
                    models: "&id, user, name, [id+user]",
                    model_patches: "$$id, model"
                });
            }
        }
        let db;
        const initializeDexie = ()=>{
            dexie.Dexie.Syncable.registerSyncProtocol("evidentstack", SupabaseSync);
            db = new EventModelDatabase();
            db.on("blocked", ()=>{
                alert("Database upgrading was blocked by another window. Please close down any other tabs or windows that has this page open");
            });
            return db;
        };
        const connect = async (url, session, statusCallback)=>{
            init_supabase(session);
            await db.syncable.connect("evidentstack", url, {
                user: session.user.id,
                local_patches_table: "model_patches",
                local_models_table: "models",
                remote_schema: "public",
                remote_events_table: "model_events",
                remote_patches_table: "model_patches"
            });
            db.syncable.on("statusChanged", function(newStatus, url_) {
                console.log("Dexie DB status changing to:", newStatus, dexie.Dexie.Syncable.StatusTexts[newStatus]);
                if (url_ == url) {
                    statusCallback(newStatus);
                }
            });
        };
        var dexie$1 = Object.freeze({
            __proto__: null,
            connect: connect,
            initializeDexie: initializeDexie
        });
    })();

})(authHelpersSveltekit, dexie, base64Js);
