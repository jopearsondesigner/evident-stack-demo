import { initFirebase } from "$lib/firebase/client";
import { initWasm } from "$lib/state";

initFirebase();
await initWasm();
