# mailBox
The mailBox is a rest API for the chicken messenger. This software should be placed on an external server somewhere apart from you. The IP address and a user address will be used to route the messages to a specific user. 

Requirements for v1.0.0
* Send encrypted messages from an account to other mailBoxes
* Receive encrypted messages from other mailBoxes 
* Send the received messages to the associated account
* Create new user accounts (You need a password for this but you have the option to make the mailBox an open mailBox)
* Create subadresses that link to your address

Data:
* Message Data
    * user_hash
    * text
    
Database:
* Users
    * user_name 
    * user_hash
    * user_password_hash

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
