# Chicken mailBox

The mailBox is a rest API for the chicken messenger. This software should be deployed on an server or an computer that is always connected to the internet. 

Requirements for v1.0.0
* Send encrypted messages from an account to other mailBoxes
* Receive encrypted messages from other mailBoxes 
* Send the received messages to the associated account

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
* Messaging:
    * put_message(user_hash, encrypted_message)
    * get_messages(user_hash)