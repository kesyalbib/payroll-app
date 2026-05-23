const CONTRACT_ID =
  "CBLZMM5IIGMFQ2EIIXBQ2S5NL7HJ4Y2B2KS4O6FOTO32BRWYUODQKTGV";

const RPC_URL =
  "https://soroban-testnet.stellar.org";

const networkPassphrase =
  StellarSdk.Networks.TESTNET;

const server =
  new StellarSdk.SorobanRpc.Server(RPC_URL);

// =====================================================
// CONNECT FREIGHTER
// =====================================================

async function connectWallet() {

  if (!window.freighterApi) {
    alert("Freighter Wallet belum terinstall");
    return null;
  }

  try {

    const publicKey =
      await window.freighterApi.getPublicKey();

    console.log("Connected:", publicKey);

    return publicKey;

  } catch (err) {

    console.error(err);
    alert("Gagal connect wallet");

    return null;
  }
}

// =====================================================
// ADD EMPLOYEE
// =====================================================

async function addEmployee() {

  try {

    const userAddress =
      await connectWallet();

    if (!userAddress) return;

    const name =
      document.getElementById("name").value;

    const wallet =
      document.getElementById("wallet").value;

    const rate =
      Number(
        document.getElementById("rate").value
      );

    const sourceAccount =
      await server.getAccount(userAddress);

    const tx =
      new StellarSdk.TransactionBuilder(
        sourceAccount,
        {
          fee: "100",
          networkPassphrase,
        }
      )
      .addOperation(
        StellarSdk.Operation.invokeHostFunction({
          func:
            StellarSdk.xdr.HostFunction.hostFunctionTypeInvokeContract(),

          parameters: [
            StellarSdk.nativeToScVal(CONTRACT_ID, {
              type: "address"
            }),

            StellarSdk.nativeToScVal("add_employee"),

            StellarSdk.nativeToScVal(wallet, {
              type: "address"
            }),

            StellarSdk.nativeToScVal(name),

            StellarSdk.nativeToScVal(rate, {
              type: "i128"
            })
          ]
        })
      )
      .setTimeout(30)
      .build();

    const prepared =
      await server.prepareTransaction(tx);

    const signedXDR =
      await window.freighterApi.signTransaction(
        prepared.toXDR(),
        {
          network: "TESTNET"
        }
      );

    const signedTx =
      StellarSdk.TransactionBuilder.fromXDR(
        signedXDR,
        networkPassphrase
      );

    const response =
      await server.sendTransaction(signedTx);

    console.log(response);

    alert("Employee berhasil ditambahkan!");

  } catch(err) {

    console.error(err);

    alert(
      "ERROR:\n" +
      JSON.stringify(err, null, 2)
    );
  }
}