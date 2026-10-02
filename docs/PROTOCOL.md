# notesync protocol

## Changes {#sp-changes}
A change replaces one whole document: its id and every field, with the device that made it, that device's sequence number, and a timestamp.

## Change timestamps {#sp-change-timestamp}
A change's timestamp is a hybrid logical clock reading: the later of the authoring device's wall clock and the newest timestamp that device has stored, plus a counter that breaks ties. A change therefore sorts after every change its author had stored when making it. The timestamp is not the time the change was made and can run ahead of any device's clock; to show when something happened, use observed_at.

## Conflict resolution {#sp-conflict-resolution}
When two changes replace the same document, the one with the higher timestamp wins; equal timestamps break on device id. Resolution is order-independent: replicas that have stored the same changes agree. A change always beats the changes its author had stored; changes made without seeing each other still race on timestamp, so a fast clock can win that race.

## Sync {#sp-sync}
Replicas exchange the changes the other has not seen, by version vector. Moving changes between replicas is the caller's concern.

## Observed time {#sp-observed-at}
observed_at is when this replica first stored a change, by this replica's clock. It is local and never sent between replicas. Logs written by v0.1 have no observed column; their observed_at is their timestamp.
