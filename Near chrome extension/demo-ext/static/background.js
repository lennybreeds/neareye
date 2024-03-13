import web3 from'@solana/web3.js';
import anchor from"@coral-xyz/anchor";
import fs from 'fs';
import {AnchorProvider, Wallet} from '@coral-xyz/anchor';
// Initialize connection to the Solana cluster
const providerUrl = 'http://127.0.0.1:8899';
const programId = new web3.PublicKey('EvWSp8yjbVF9vC7BrBWzejhS6GsqcyE3hELv54r614Yt'.trim());
// Create a connection to the Solana cluster
const connection = new web3.Connection(providerUrl, 'processed');

const walletKeypairPath = '/Users/lennybreeds/Desktop/Computingprojects/SolEye/front-end/wallet-keypair.json';

// Load the wallet keypair
const keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(walletKeypairPath, 'utf-8')))
);

const wallet = new Wallet(keypair);

 const idl = JSON.parse(fs.readFileSync('/Users/lennybreeds/Desktop/Computingprojects/SolEye/front-end/idl.json', 'utf8'));
 const provider = new AnchorProvider(connection, wallet, {
    preflightCommitment: 'processed',
  });
  anchor.setProvider(provider);
  
  
    
    // Step 3: Fetch Latest Blockhash
  
    // Step 4: Send and Confirm the Transaction
    // Adjust `addWebsite` method parameters according to your actual program's method signature
  const program = new anchor.Program(idl, programId, provider);

chrome.runtime.onMessage.addListener(function (message, sender, senderResponse) {
    //Check solana for website data
    async function fetchWebsite(url) {
        const latestBlockhash = await provider.connection.getLatestBlockhash("finalized");
          const [websitePda] = await anchor.web3.PublicKey.findProgramAddress(
              [Buffer.from(url)],
              programId
          );
      
          // Fetch account information
              const accountInfo = await connection.getAccountInfo(websitePda);
              if (accountInfo === null || accountInfo == {}) {
                  return "Failed";
              }
          // Deserialize the account data
              try {
                  const accountData = await program.account.yourAccountType.fetch(accountPublicKeyObj);
                  console.log(accountData);
              }catch(e){
                  return "Failed"
              }
              return accountData;
      }
    const accountData = fetchWebsite(message.url);
  if (accountData == "Failed") {
    //Add website to backend which will then add to blockchain
        if (message.req === "safety") {
            // console.log('getting safety')
            fetch('http://127.0.0.1:8000/check-safety/', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ "url": message.url, "content": message.content, "name": message.url, "code": message.code })
            }).then(res => {
                return res.json();
            }).then(res => {
                // console.log(res)
                senderResponse(res);
            })
            // console.log('send req')
            return true
        }
  } 
    });