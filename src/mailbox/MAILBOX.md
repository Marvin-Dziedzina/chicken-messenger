# Chicken mailBox

The mailBox receives messages if the recipient is not online. If the recipient is online the collected messages get sent to the user.

The mailBox shouldnt store important data and the data that is stored for the mailBox to function should be deleted as soon as all recipients have collected the data. The mailBox will receive and send all data over tor.

Requirements for v1.0.0
* Routed over tor
* Collect encrypted messages from an client 
* Send the collected messages to the recipients
* Delete all stored data associated to the message when it was collected by all recipients

Data:
* Message Data
    * user_hash
    * encrypted message

Database:
* Messages
    * user_hash
        * messages
        * messages
        * messages
        * ...
    * user_hash
        * messages
        * messages
        * messages
        * ...

Endpoints:
  * put_message(user_hash, encrypted_message) -> OK?
  * get_messages(user_hash) -> Messages
  * is_new_message_available(user_hash) -> Number of new messages


Structure:
* Chat|Group Chat:
  * Messages contains data:
    - Text
    - Files
    - Pictures
