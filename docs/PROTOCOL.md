# notesync protocol

## Changes {#sp-changes}
A change replaces one whole document: its id and every field, with the device that made it, that device's sequence number, and a timestamp.

## Change timestamps {#sp-change-timestamp}
A change's timestamp is the authoring device's wall-clock time when the change was made, in milliseconds since the Unix epoch.

## Conflict resolution {#sp-conflict-resolution}
When two changes replace the same document, the one with the higher timestamp wins; equal timestamps break on device id. Resolution is order-independent: replicas that have stored the same changes agree. Device clocks are trusted, so a device whose clock runs fast wins conflicts it should lose.

## Sync {#sp-sync}
Replicas exchange the changes the other has not seen, by version vector. Moving changes between replicas is the caller's concern.
