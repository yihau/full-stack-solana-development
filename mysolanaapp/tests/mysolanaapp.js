const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("mysolanaapp", () => {
  /* create and set a Provider */
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  it("創建一個帳戶", async () => {
    // 定義program是我們的mysolanaapp
    const program = anchor.workspace.Mysolanaapp;
    // 這邊用內建的隨意創一個
    const baseAccount = anchor.web3.Keypair.generate();
    // 這邊規則跟之前說的一樣，可以使用 program.rpc.<instruction-name-in-program> 來呼叫
    await program.rpc.create({
      accounts: {
        // 這邊的輸入會跟我們在program裡面定義的context是一樣的
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      // baseAccount會需要簽名是因為他要被創建
      // 不太熟悉的人可以去我的solana-web3-demo的tour過一下概念
      signers: [baseAccount],
    });

    // 驗證我們創出來的account可以成功被讀取資料
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 0: ', account.count.toString())
    assert.ok(account.count.toString() == 0);
    _baseAccount = baseAccount;

  });

  it("增加", async () => {
    // 這邊延續我們剛剛創出來的account
    const baseAccount = _baseAccount;
    // 一樣定義是我們的program
    const program = anchor.workspace.Mysolanaapp;
    // 這邊規則跟之前說的一樣，可以使用 program.rpc.<instruction-name-in-program> 來呼叫
    await program.rpc.increment({
      accounts: {
        // 如同我們program定義的increment的context
        baseAccount: baseAccount.publicKey,
      },
    });

    // 驗證我們的+1有沒有成功
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 1: ', account.count.toString())
    assert.ok(account.count.toString() == 1);
  });
});