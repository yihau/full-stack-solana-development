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
