# The SOLFS DATASTORE

* Data Account: This represents the main storage where the actual data is kept. It's owned by the Data Program, which is the smart contract managing the storage.

* Data Store PDA (Program Derived Address): This acts as an intermediary or a linkage between the Data Account and the Metadata Account. It contains references to the data account, its program ID (dataAccountPID), and the program ID of the data program (dataProgramPID). This PDA helps in managing permissions and linking different parts of the system.

* Metadata Account: This account holds metadata about the data stored in the Data Account. It includes various attributes like:
    - Data Status: Indicates the current status of the data (e.g., active, archived).
    - Serialization Status: Information on how the data is serialized or formatted.
    - Authority: The account that has control over this data store.
    - Dynamic: Possibly indicates if the data can be dynamically updated.
    - Data Version: Version control for the data.
    - Data Type: The type of data stored.
    - Bump: Used in Solana for PDA derivation to ensure uniqueness.

* Authority Wallet: This represents the wallet of the user or entity that has the authority to interact with the Data Account and Metadata Account. It must sign transactions for operations like initializing, updating, finalizing, or closing the data store.





```mermaid
graph TD
    A[Data Account<br>Owner: Data Program] -->|PDA| B[Data Store PDA<br>+data_account<br>+dataAccountPID<br>+dataProgramPID]
    B -->|Metadata| C[Metadata Account<br>Owner: Data Program<br>- Data Status<br>- Serialization Status<br>- Authority<br>- Dynamic<br>- Data Version<br>- Data Type<br>- Bump]
    D[Authority Wallet] -.->|Signer| A
    D -.->|Signer| C