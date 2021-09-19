# Solana全端開發

這是基於[這篇](https://dev.to/dabit3/the-complete-guide-to-full-stack-solana-development-with-react-anchor-rust-and-phantom-3291)做的一些濃縮整理。

## 專案概述

這個應用會用到以下的工具

### [Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools)

這是Solana官方的工具，裡面把常用的指令都包裝好了。

### [Anchor Framework](https://project-serum.github.io/anchor/getting-started/introduction.html)

Anchor就像是Hardhat,Truffle這類的工具。他還在Rust的更上層提供了DSL，讓你可以在不是很熟悉Rust的同時也能開始開發！但也是滿建議能在有空之於學習一下Rust，這邊有一個[滿好的地方可以學習](https://doc.rust-lang.org/book/title-page.html)

### [solana/web3.js](https://solana-labs.github.io/solana-web3.js/)

就是Solana版本的[web3.js](https://web3js.readthedocs.io)，另外我有一篇[solana-web3-demo](https://github.com/yihau/solana-web3-demo)可以搭配使用。

### [React](https://reactjs.org/)

無須多做介紹，很熱門的前端框架。

這邊會主要專注在開發而不會對Solana進行太深入地講解。但如果你想要更了解Solana的話，可以參考下面幾篇

- [Solana Docs Introduction](https://docs.solana.com/introduction)
- [ok so what the fuck is the deal with solana anyway](https://2501babe.github.io/posts/solana101.html)
- [Solana Summer](https://www.notboring.co/p/solana-summer)

以上是原作者列舉的文章，我自己則是非常推薦一定要讀懂[Programming Model](https://docs.solana.com/developing/programming-model/overview)，因為Solana的Account機制和Ethereum非常不一樣，會需要一些時間習慣。而這篇裡面所講的東西在開發任何Solana應用都會用到。

另外如果是有想要關注NFT開發的人，可以關注[Metaplex](https://www.metaplex.com/)

## 準備

在開始前可能會需要預先安裝一些東西

1. Node.js (推薦使用nvm或是fnm來安裝)
2. Solana Tool Suite ([這裡](https://docs.solana.com/cli/install-solana-cli-tools)有安裝說明，另外如果是M1的話，可以參考[這裡](https://github.com/project-serum/anchor/issues/95#issuecomment-913090162))
3. Anchor ([這裡](https://project-serum.github.io/anchor/getting-started/installation.html#install-rust)有安裝步驟)
4. Solana browser wallet (推薦使用[Phontom](https://phantom.app/))

## 開始

### Solana CLI

```sh
solana config get
```

這可以看一下當前solana cli工具的config設定。如果你還沒有設置key的話可以來[這裡](https://docs.solana.com/wallet-guide/paper-wallet#seed-phrase-generation)

如果想要替換連接的網路的話可以使用

```sh
# 換到 localhost
solana config set --url localhost

# 換到 devnet
solana config set --url devnet

# 或是你也可以使用簡寫

# 換到 localhost
solana config set -ul

# 換到 devnet
solana config set -ud
```

隨時注意自己的連接網路很重要，避免用到其他環境造成奇怪的結果。

這邊我們先切到devnet來方便下面兩個指令的操作。

當前的地址

```sh
solana address
```

帳戶詳細的資訊

```sh
solana account <上面的地址>
```

再來我們切換到localhost來進行設置

```sh
# 切換回localnet
solana config set -ul

# 執行本地節點
# 如果是Windows用戶，目前還不支援這個指令
solana-test-validator
```

當本地節點跑起來的時候我們可以拿一些SOL的airdrop

```sh
solana aridrop 100
```

SOL金額

```sh
solana balance

# or

solana balance <地址>
```

如果一切都進行得很順利的話，你目前應該會有100SOL🤑 (的測試代幣🥲)

## Anchor

我們可以用下面指令來使用Anchor啟動一個新專案

```sh
anchor init mysolanaapp

cd mysolanaapp
```

在這個專案裡面應該會是長得像這樣的結構

```sh
.
├── Anchor.toml # anchor的設定檔
├── Cargo.toml # cargo的設定檔
├── app # 我們的前端code會在這邊
├── migrations # 可以設置deploy script
├── programs # Solana Program 會在這裡
└── tests # 寫測試的地方
```

我們先來看一下他內建幫我們產好的program。

Anchor使用了[eDSL](https://en.wikipedia.org/wiki/Domain-specific_language#:~:text=embedded%20domain%2Dspecific%20language%20(eDSL,methods%2C%20macros%20etc.))，他簡化了很多複雜的底層操作，
讓code變的更容易讀。

*programs/mysolanaapp/src/lib.rs*
```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mysolanaapp {
    use super::*;
    // 這個是你program定義的操作
    // 目前只有 initialize 可以提供給使用者呼叫
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}
// 這個是要呼叫的參數設置，這邊我們只有initialize的function有用到它
#[derive(Accounts)]
pub struct Initialize {}
```

這應該是anchor裡面最基本的program，這個program目前只有提供`initialize`的操作，並沒有任何data的更新。

`Initialize`的struct定義context，我們晚點會在這邊多介紹一些。

要編譯這個program，我們可以使用

```sh
anchor build
```

當你編譯完成後，你應該會看到一個新的資料夾 `target`。其中有一個很重要的檔案會在
*target/idl/mysolanaapp.json*。這個是[IDL](https://en.wikipedia.org/wiki/Interface_description_language)。

可以把IDL看作是Solana的[ABI](https://docs.soliditylang.org/en/v0.5.3/abi-spec.html)，就是一個定義query介面的一個描述檔案。

另外我們還可以測試我們的Program。

*tests/mysolanaapp.js*
```js
const anchor = require('@project-serum/anchor');

describe('mysolanaapp', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Mysolanaapp;
    const tx = await program.rpc.initialize();
    console.log("Your transaction signature", tx);
  });
});
```

這邊有幾個東西需要特別介紹一下。

### Provider

這個是對solana連線的一個抽象，是由connection, wallet和preflight commitment組成。

在測試裡面anchor會用`anchor.Provider.env`來設置provider，不過如果我們現在是要寫client app的話，會需要改用user的solana錢包來設置。

### Program

這是對`Provider`和`idl`以及`programID`的抽象。並且它允許你呼叫`RPC`方法。

跟provider一樣，我們在client app的設置也需要注意。

當我們準備好這兩個東西後，我們就可以開始和program交互。因為我們在program裡面有`initialize`，所以我們可以使用

```js
const tx = await program.rpc.initialize();
```

來直接與program互動，而使用規則的話通常都是`program.rpc.functionName`

晚點看更多例子的時候我們可以有更深刻的體會。現在我們先來執行看看這個test。

```sh
anchor test
```

沒意外的話他會噴一個警告，跟你說你的ctx沒有用，要改成_ctx，這樣這邊的基本操作就算是完成了。我們接下來要來打造我們第一個Hello World

## 建置Hello World

這邊拿剛剛的專案來修改，我們會做一個計數器，每次被呼叫的時候都會+1。

*programs/mysolanaapp/src/lib.rs*
```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod mysolanaapp {
    use super::*;

    // 因為Solana account model的關係，我們需要創造一個帳戶來儲存
    // 我們的計數結果，而不是直接把數字存在合約中
    // 這邊我們定義一個create的操作，讓帳戶能在這個合約內被初始化
    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    // 這個操作就是+1的地方，這邊會取client傳過來的計數用的帳戶
    // 然後對他+1
    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

// 這個是create操作時所需要的一些參數
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

// 這個是increment所需要的參數
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// 儲存數量的結構體
#[account]
pub struct BaseAccount {
    pub count: u64,
}
```

在這個program內有兩個instruction，`create`和`increment`。

一般我們在新建insturction時都會需要傳入一個Context的結構，主要就是定義這個instruction會用到什麼東西。

`#[account(...)]` 是一個對於acccount的加強描述，他會定義這個account在這個instruction的限制，如果傳入的account不滿足這些敘述，那這個instruction就會失敗。

所以以這個例子來說，我們並沒有定義誰擁有什麼帳戶，也沒有相關的驗證權限，也就是說在我們現在的program內，Alice是可以拿Bob創出的account的。

完成之後記得再下一次build指令

```sh
anchor build
```

接下來我們來寫test

*tests/mysolanaapp.js*

```js
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
```

在我們執行它之前，我們會需要知道我們的program ID，我們可以透過下面的指令得到它

```
solana address -k target/deploy/mysolanaapp-keypair.json
```

並且在

*mysolanaapp/src/lib.rs*

```rust
// 把原本在裡面的數值換成我們的program id
declare_id!("your-program-id");
```

和

*Anchor.toml*

```toml
[programs.localnet]
mysolanaapp = "your-program-id"
```

上面兩步驟都完成之後，就可以來試試他了

```
anchor test
```

接下來我們來寫前端

我們先回到我們的mysolanaapp的anchor專案根目錄，用

```sh
npx create-react-app app
```

來覆蓋原本他給我們的app資料夾，接下來

```sh
cd app

npm install @project-serum/anchor @solana/web3.js
```

再來因為我們的前端會用到[Solana Wallet Adapter](https://github.com/solana-labs/wallet-adapter)，這個庫可以幫我們處理使用者的錢包，而且裡面還集成了很多其他大宗的錢包。他需要的套件有下面這些，我們也把他裝起來。

```sh
npm install @solana/wallet-adapter-react \
@solana/wallet-adapter-react-ui @solana/wallet-adapter-wallets \
@solana/wallet-adapter-base
```

裝完之後我們把IDL檔案複製過來

```
cp ../target/idl/mysolanaapp.json src/idl.json
```

接下來我們來改前端的頁面

*app/src/App.js*

```js
import './App.css';
import { useState } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import {
  Program, Provider, web3
} from '@project-serum/anchor';
import idl from './idl.json';

import { getPhantomWallet } from '@solana/wallet-adapter-wallets';
import { useWallet, WalletProvider, ConnectionProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';

const wallets = [
  /* view list of available wallets at https://github.com/solana-labs/wallet-adapter#wallets */
  getPhantomWallet()
]

const { SystemProgram, Keypair } = web3;
/* create an account  */
const baseAccount = Keypair.generate();
const opts = {
  preflightCommitment: "processed"
}
const programID = new PublicKey(idl.metadata.address);

function App() {
  const [value, setValue] = useState(null);
  const wallet = useWallet();

  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const network = "http://127.0.0.1:8899";
    const connection = new Connection(network, opts.preflightCommitment);

    const provider = new Provider(
      connection, wallet, opts.preflightCommitment,
    );
    return provider;
  }

  async function createCounter() {
    const provider = await getProvider()
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program(idl, programID, provider);
    try {
      /* interact with the program via rpc */
      await program.rpc.create({
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount]
      });

      const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
      console.log('account: ', account);
      setValue(account.count.toString());
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  async function increment() {
    const provider = await getProvider();
    const program = new Program(idl, programID, provider);
    await program.rpc.increment({
      accounts: {
        baseAccount: baseAccount.publicKey
      }
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('account: ', account);
    setValue(account.count.toString());
  }

  if (!wallet.connected) {
    /* If the user's wallet is not connected, display connect wallet button. */
    return (
      <div style={{ display: 'flex', justifyContent: 'center', marginTop:'100px' }}>
        <WalletMultiButton />
      </div>
    )
  } else {
    return (
      <div className="App">
        <div>
          {
            !value && (<button onClick={createCounter}>Create counter</button>)
          }
          {
            value && <button onClick={increment}>Increment counter</button>
          }

          {
            value && value >= Number(0) ? (
              <h2>{value}</h2>
            ) : (
              <h3>Please create the counter.</h3>
            )
          }
        </div>
      </div>
    );
  }
}

/* wallet configuration as specified here: https://github.com/solana-labs/wallet-adapter#setup */
const AppWithProvider = () => (
  <ConnectionProvider endpoint="http://127.0.0.1:8899">
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <App />
      </WalletModalProvider>
    </WalletProvider>
  </ConnectionProvider>
)

export default AppWithProvider;
```

改完之後，我們會需要記得把phantom裡面的network也改成localnet

![1](https://res.cloudinary.com/practicaldev/image/fetch/s--TUVVaCuV--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/dw09ddfv8sf96px7clc5.png)
![2](https://res.cloudinary.com/practicaldev/image/fetch/s--7fFmF46U--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/b8uxbjhqhnvnuheal50v.png)

再來我們要來幫phontom的地址拿一點airdrop

![address](https://res.cloudinary.com/practicaldev/image/fetch/s--A6wOufNS--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_880/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/746cr3yu2gprby424w1w.png)

點一下這邊就會複製了，接下來回到command line，記得你的solana-test-validator要開起來!

```
solana airdrop 10 <phantom地址>
```

回到我們的前端專案(app/)執行

```
npm start
```

你會發現當你完成操作再次刷新頁面時，剛剛產生的地址就不見了。這是因為我們每次都是隨機的，所以計數器的地址和我們的帳號地址沒有關聯性。想要解決這個事情原作者有提供一個[gist](https://gist.github.com/dabit3/7cbd18b8bc4b495c4831f8674902eb42)。

我自己則是建議你能夠設計一個計數器帳號和使用者帳號的關聯，
可以使用[findProgramAddress](https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html#findProgramAddress)，這可以傳seed並且計算PDA，有興趣的朋友可以往這方面研究一下。

## Hello World part 2

再來我們會建一個能夠儲存訊息的program, 這邊你可以用原本的專案繼續改，也可以創一個新的。

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod helloworld2 {
    use super::*;
    // init的操作
    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
    // 更新資料
    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// 儲存訊息的結構
#[account]
pub struct BaseAccount {
    // 當前資料
    pub data: String,
    // 歷史資料
    pub data_list: Vec<String>,
}
```

這邊的space是64+64，這個大小是可以自訂的，完全依照自己的需求來給。不過一旦固定之後之後要換到更大的space的account會需要多寫migration。

接下來是test

```js
const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("Mysolanaapp", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  it("It initializes the account", async () => {
    const program = anchor.workspace.Mysolanaapp;
    const baseAccount = anchor.web3.Keypair.generate();
    await program.rpc.initialize("Hello World", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Data: ', account.data);
    assert.ok(account.data === "Hello World");
    _baseAccount = baseAccount;

  });

  it("Updates a previously created account", async () => {
    const baseAccount = _baseAccount;
    const program = anchor.workspace.Mysolanaapp;

    await program.rpc.update("Some new data", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Updated data: ', account.data)
    assert.ok(account.data === "Some new data");
    console.log('all account data:', account)
    console.log('All data: ', account.dataList);
    assert.ok(account.dataList.length === 2);
  });
});
```

```
anchor test
```

最後是前端的code

```js
import './App.css';
import { useState } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, Provider, web3 } from '@project-serum/anchor';
import idl from './idl.json';

import { getPhantomWallet } from '@solana/wallet-adapter-wallets';
import { useWallet, WalletProvider, ConnectionProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui';

const wallets = [ getPhantomWallet() ]

const { SystemProgram, Keypair } = web3;
const baseAccount = Keypair.generate();
const opts = {
  preflightCommitment: "processed"
}
const programID = new PublicKey(idl.metadata.address);

function App() {
  const [value, setValue] = useState('');
  const [dataList, setDataList] = useState([]);
  const [input, setInput] = useState('');
  const wallet = useWallet()

  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const network = "http://127.0.0.1:8899";
    const connection = new Connection(network, opts.preflightCommitment);

    const provider = new Provider(
      connection, wallet, opts.preflightCommitment,
    );
    return provider;
  }

  async function initialize() {
    const provider = await getProvider();
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program(idl, programID, provider);
    try {
      /* interact with the program via rpc */
      await program.rpc.initialize("Hello World", {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount]
      });

      const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
      console.log('account: ', account);
      setValue(account.data.toString());
      setDataList(account.dataList);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  async function update() {
    if (!input) return
    const provider = await getProvider();
    const program = new Program(idl, programID, provider);
    await program.rpc.update(input, {
      accounts: {
        baseAccount: baseAccount.publicKey
      }
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('account: ', account);
    setValue(account.data.toString());
    setDataList(account.dataList);
    setInput('');
  }

  if (!wallet.connected) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', marginTop:'100px' }}>
        <WalletMultiButton />
      </div>
    )
  } else {
    return (
      <div className="App">
        <div>
          {
            !value && (<button onClick={initialize}>Initialize</button>)
          }

          {
            value ? (
              <div>
                <h2>Current value: {value}</h2>
                <input
                  placeholder="Add new data"
                  onChange={e => setInput(e.target.value)}
                  value={input}
                />
                <button onClick={update}>Add data</button>
              </div>
            ) : (
              <h3>Please Inialize.</h3>
            )
          }
          {
            dataList.map((d, i) => <h4 key={i}>{d}</h4>)
          }
        </div>
      </div>
    );
  }
}

const AppWithProvider = () => (
  <ConnectionProvider endpoint="http://127.0.0.1:8899">
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <App />
      </WalletModalProvider>
    </WalletProvider>
  </ConnectionProvider>
)

export default AppWithProvider;
```

再來記得確定你的 `solana-test-validator` 有跑起來。執行

```
anchor build

anchor deploy
```

記得一樣要把idl檔案複製到app的src下面

```
npm start
```

## Deploying to devnet

我們也可以把這個program部署到devnet上面

1. 先把solana config連接到devnet

```
solana config set -ud
```

2. 更新你的phantom連接的網路到devnet

3. 打開 **Anchor.toml**，把localnet改成devnet

4. 重新build一次program並且確認一下program id是不是都有改好

5. 重新下一次deploy指令，這次我們就會部署到devnet上了

6. 記得要修改App.js內的連接網路

```js
// 修改前
<ConnectionProvider endpoint="http://127.0.0.1:8899">

// 修改後
import {
  ...,
  clusterApiUrl
} from '@solana/web3';

const network = clusterApiUrl('devnet');

<ConnectionProvider endpoint={network}>
```

