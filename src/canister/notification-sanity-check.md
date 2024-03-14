### Calls that use notification and flow

###### good-to-know

- notifications are stored on the canister
- websocket messages not stored

###### By default websocket messages are NOT SILENT

- which means that they trigger a visual action on the frontend like a toasty or visual indicator.

###### When a websocket message is SILENT

- it should be used to update a state without the user noticing, for example when somebody leaves the group and the member count gets updated in the background.

## Friend requests

#### (1A) add_friend_request

1a.1 - create friend request
1a.2 - create actionable notification
1a.3 - add notification id to friend request (1a.1)
1a.4 - store friend request in friend request store
1a.5 - send notification as websocket message to receiver (1a.2)

#### (1B) accept_friend_request

1b.1 - add friend to relations hashmap on the user profile
1b.2 - add user to relations hashmap on the friend profile
1b.3 - remove friend request from friend request store (1a.1)
1b.4 - update actionable notification as accepted and change actionable to false (1a.2)
1b.5 - create and store notification that the request is accepted.
1b.6 - send websocket message with the notification to the sender (1b.5)

#### (1C) decline_friend_request

1c.1 - remove friend request from friend request store
1c.2 - update actionable notification as declined and change actionable to false
1c.3 - send websocket message to user that the friend request is a declined

#### (1D) remove_friend

1d.1 - remove friend from profile relations
1d.3 - send websocket message to removed friend to update the friends list - **(SILENT)**

## Group join requests

#### (2A) join_group (public)

2a.1 - add group join to member object
2a.2 - send silent websocket message to group members to update the members

#### (2B) join_group (private) **(not silent)**

2b.1 - create UserRequest group invite for member object
2b.2 - create actionable notification
2b.3 - add notification id to group invite (2b.1)
2b.4 - add new invite to member
2b.5 - send websocket message to group members with Invite write permissions

#### (2C) accept_user_request_group_invite

2c.1 - add join entry to joined on the member
2c.2 - remove UserRequest invite member for group
2c.3 - update actionable notification as accepted and change actionable to false (2b.2)
2c.4 - send websocket message to the member to notifify the accept
2c.5 - send websocket message to group members with Invite write permissions to update the actionable notification **(silent)**
2c.6 - send websocket message to group members to update the members **(silent)**

#### (2D) decline_user_request_group_invite

2d.1 - remove UserRequest invite member for group
2d.2 - update actionable notification as declined and change actionable to false
2d.3 - send websocket message to the member to notifify the decline

## Group invite requests

#### (3A) invite_to_group

3a.1 - create OwnerRequest group invite for member object
3a.2 - create actionable notification
3a.3 - add notification id to group invite (3a.1)
3a.4 - add new invite to member
3a.5 - send websocket message to invited member

#### (3B) accept_owner_request_group_invite

3b.1 - add join entry to joined on the member
3b.2 - remove OwnerRequest invite member for group
3b.3 - update actionable notification as accepted and change actionable to false
3b.4 - send websocket message to sender notifying the accept
3b.5 - send websocket message to group members to update the members **(silent)**

#### (3C) decline_owner_request_group_invite

3c.1 - remove OwnerRequest invite member for group
3c.2 - update actionable notification as declined and change actionable to false
3c.3 - send websocket message to the sender to notifify the decline

## Event join requests

#### (4A) join_event (public)

4a.1 - add event join to attendee object
4a.2 - send silent websocket message to event owner

#### (4B) join_event (private)

4b.1 - create UserRequest event invite for attendee object
4b.2 - create actionable notification
4b.3 - add notification id to event invite (4b.1)
4b.4 - add new invite to attendee
4b.5 - send websocket message to event owner

#### (4C) accept_user_request_event_invite

4c.1 - add join entry to joined on the attendee
4c.2 - remove UserRequest invite attendee for event
4c.3 - update actionable notification as accepted and change actionable to false (4b.2)
4c.4 - send websocket message to the attendee to notifify the accept

#### (4D) decline_user_request_event_invite

4d.1 - remove UserRequest invite attendee for event
4d.2 - update actionable notification as declined and change actionable to false
4d.3 - send websocket message to the attendee to notifify the decline

## Event invite requests

#### (5A) invite_to_event

5a.1 - create OwnerRequest event invite for attendee object
5a.2 - create actionable notification
5a.3 - add notification id to event invite (5a.1)
5a.4 - add new invite to attendee
5a.5 - send websocket message to invited attendee

#### (5B) accept_owner_request_event_invite

5b.1 - add join entry to joined on the attendee
5b.2 - remove OwnerRequest invite attendee for event
5b.3 - update actionable notification as accepted and change actionable to false
5b.4 - send websocket message to sender notifying the accept

#### (5C) decline_owner_request_event_invite

5c.1 - remove OwnerRequest invite attendee for event
5c.2 - update actionable notification as declined and change actionable to false
5c.3 - send websocket message to the sender to notify the decline

## Multisig

## Transactions

## Tipping
