import anyTest from 'ava';
import { KeyPair, NEAR, Worker } from 'near-workspaces';
import { getKeyFromFile } from 'near-workspaces/dist/account/utils.js';
import { setDefaultResultOrder } from 'dns'; setDefaultResultOrder('ipv4first'); // temp fix for node >v17

/**
 *  @typedef {import('near-workspaces').NearAccount} NearAccount
 *  @type {import('ava').TestFn<{worker: Worker, accounts: Record<string, NearAccount>}>}
 */
const test = anyTest;

test.beforeEach(async t => {
  // Create sandbox
  const worker = t.context.worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount('test-account');

  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );

  // Save state for test runs, it is unique for each test
  t.context.accounts = { root, contract };
});

test.afterEach.always(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('returns the default greeting', async (t) => {
  const { contract } = t.context.accounts;
  const greeting = await contract.view('get_greeting', {});
  t.is(greeting, 'Hello');
});

test('changes the greeting', async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, 'set_greeting', { greeting: 'Howdy' });
  const greeting = await contract.view('get_greeting', {});
  t.is(greeting, 'Howdy');
});

test('create dev account', async (t) => {
  const { root } = t.context.accounts;
  const devUser = await root.devCreateAccount();
  
  console.log('devUser:', devUser);
  // devUser: Account {
  //   _accountId: 'dev-32502-22496.test.near',
  //   manager: <ref *1> SandboxManager {
  //     config: {
  //       network: 'sandbox',
  //       rootAccountId: 'test.near',
  //       rpcAddr: 'http://127.0.0.1:21879',
  //       initialBalance: '100000000000000000000000000',
  //       homeDir: '/private/var/folders/yf/n8735nxj4qz69g1ck3zchh680000gn/T/sandbox/bd47fd10-f3ad-4253-a828-47a599de2bbd',
  //       port: 21879,
  //       rm: false,
  //       refDir: null
  //     },
  //     accountsCreated: Set(2) { 'test-account.test.near', 'dev-32502-22496.test.near' },
  //     _root: Account { _accountId: 'test.near', manager: [Circular *1] }
  //   }
  // }

  t.regex(devUser.accountId, /dev-\b\d+\b-\b\d+\b.test.near/)
});

test('create subaccount', async (t) => {
  const { root } = t.context.accounts;
  const subaccount = await root.createSubAccount('subaccount');
  
  console.log('subaccount:', subaccount);
  // subaccount: Account {
  //   _accountId: 'subaccount.test.near',
  //   manager: <ref *1> SandboxManager {
  //     config: {
  //       network: 'sandbox',
  //       rootAccountId: 'test.near',
  //       rpcAddr: 'http://127.0.0.1:5014',
  //       initialBalance: '100000000000000000000000000',
  //       homeDir: '/private/var/folders/yf/n8735nxj4qz69g1ck3zchh680000gn/T/sandbox/f6ebeb0b-ecb9-4233-b6df-facbb9861b15',
  //       port: 5014,
  //       rm: false,
  //       refDir: null
  //     },
  //     accountsCreated: Set(2) { 'test-account.test.near', 'subaccount.test.near' },
  //     _root: Account { _accountId: 'test.near', manager: [Circular *1] }
  //   }
  // }

  t.is(subaccount.accountId, 'subaccount.test.near')
});

test('create account using secret key', async (t) => {
  const { root } = t.context.accounts;

  const account_id = "secret.test.near";
  const keyPair = KeyPair.fromRandom("ED25519");

  const account = await root.createAccount(account_id, {
    keyPair,
    initialBalance: "100000000000000000000000",
  });
  
  console.log('account from secret key:', account);
  // account from secret key: Account {
  //   _accountId: 'secret.test.near',
  //   manager: <ref *1> SandboxManager {
  //     config: {
  //       network: 'sandbox',
  //       rootAccountId: 'test.near',
  //       rpcAddr: 'http://127.0.0.1:28270',
  //       initialBalance: '100000000000000000000000000',
  //       homeDir: '/private/var/folders/yf/n8735nxj4qz69g1ck3zchh680000gn/T/sandbox/04b4f82d-6232-4a87-8c7d-3396f1c3eb57',
  //       port: 28270,
  //       rm: false,
  //       refDir: null
  //     },
  //     accountsCreated: Set(2) { 'test-account.test.near', 'secret.test.near' },
  //     _root: Account { _accountId: 'test.near', manager: [Circular *1] }
  //   }
  // }

  t.is(account.accountId, 'secret.test.near')
});

test('create account from credentials file', async (t) => {
  const { root } = t.context.accounts;

  const account_id = "file.test.near";
  const keyPair = await getKeyFromFile('.near-credentials/workspaces/testnet/getKeyFromFile.json');

  const account = await root.createAccount(account_id, {
    keyPair,
    initialBalance: "100000000000000000000000",
  });
  
  console.log('account with credentials from file:', account);
  // account with credentials from file: Account {
  //   _accountId: 'file.test.near',
  //   manager: <ref *1> SandboxManager {
  //     config: {
  //       network: 'sandbox',
  //       rootAccountId: 'test.near',
  //       rpcAddr: 'http://127.0.0.1:59952',
  //       initialBalance: '100000000000000000000000000',
  //       homeDir: '/private/var/folders/yf/n8735nxj4qz69g1ck3zchh680000gn/T/sandbox/f8b61465-4fb7-48ae-9005-fcaa79d6ab62',
  //       port: 59952,
  //       rm: false,
  //       refDir: null
  //     },
  //     accountsCreated: Set(2) { 'test-account.test.near', 'file.test.near' },
  //     _root: Account { _accountId: 'test.near', manager: [Circular *1] }
  //   }
  // }

  t.is(account.accountId, 'file.test.near')
});

test('dev deploy', async (t) => {
  const { root } = t.context.accounts;

  const contract = await root.devDeploy('./build/hello_near.wasm');
  
  console.log('contract:', contract);
  // contract: Account {
  //   _accountId: 'dev-5878-35168.test.near',
  //   manager: <ref *1> SandboxManager {
  //     config: {
  //       network: 'sandbox',
  //       rootAccountId: 'test.near',
  //       rpcAddr: 'http://127.0.0.1:42499',
  //       initialBalance: '100000000000000000000000000',
  //       homeDir: '/private/var/folders/yf/n8735nxj4qz69g1ck3zchh680000gn/T/sandbox/f5106fc1-2495-48dc-8db3-b4b587215109',
  //       port: 42499,
  //       rm: false,
  //       refDir: null
  //     },
  //     accountsCreated: Set(2) { 'test-account.test.near', 'dev-5878-35168.test.near' },
  //     _root: Account { _accountId: 'test.near', manager: [Circular *1] }
  //   }
  // }

  t.regex(contract.accountId, /dev-\b\d+\b-\b\d+\b.test.near/)
});

test('get account balance', async (t) => {
  const { root } = t.context.accounts;

  const account = await root.devCreateAccount();
  const balance = await account.balance();
  
  console.log('balance:', balance.total.toHuman());
  // balance: {
  //   total: <BN: 52b7d2dcc80cd2e4000000>,
  //   stateStaked: <BN: 62a992e53a0af00000>,
  //   staked: <BN: 0>,
  //   available: <BN: 52b77033352798d9100000>
  // }

  t.is(balance.total.toHuman(), '100 N');
});

test('patch state', async (t) => {
  const { contract } = t.context.accounts;
  const greeting = await contract.view('get_greeting', {});
  t.is(greeting, 'Hello');

  const new_greeting = 'Howdy';

  await contract.patchState('STATE', JSON.stringify({"greeting": new_greeting}));

  const updated_greeting = await contract.view('get_greeting', {});

  console.log('updated_greeting:', updated_greeting);
  // updated_greeting: Howdy

  t.is(updated_greeting, new_greeting);
});

test('time travel', async (t) => {
  const { root } = t.context.accounts;
  const { worker } = t.context;
  const simpleContract = await root.devDeploy('./sandbox-test/contracts/simple_contract.wasm');

  const [initialTimestamp, initialEpochHeight] = await simpleContract.view('current_env_data', {});
  console.log(`initialTimestamp = ${initialTimestamp}, initialEpochHeight = ${initialEpochHeight}`);

  const initialBlockInfo = await worker.provider.block({ finality: 'final' });
  console.log('initialBlockInfo:', initialBlockInfo);

  const delta = 10000;
  await worker.provider.fastForward(delta);

  const [finalTimestamp, finalEpochHeight] = await simpleContract.view('current_env_data', {});
  console.log(`finalTimestamp = ${finalTimestamp}, finalEpochHeight = ${finalEpochHeight}`);

  const finalBlockInfo = await worker.provider.block({ finality: 'final' });
  console.log('finalBlockInfo:', finalBlockInfo);

  // Rounding off to nearest hundred, providing wiggle room incase not perfectly `forward_height`
  t.true(Math.ceil(finalBlockInfo.header.height / 100) * 100 === delta);
});

test('use testnet', async (t) => {
  const testnet = await Worker.init({ network: 'testnet', testnetMasterAccountId: 'test-ac-1719933221123-3.testnet', initialBalance: NEAR.parse("0.1 N").toString()});

  const currentGreetingOnTestnet = await testnet.rootAccount.call('hello.near-examples.testnet', 'get_greeting', {});
  console.log('currentGreetingOnTestnet:', currentGreetingOnTestnet);

  const newGreeting = `howdy`;

  await testnet.rootAccount.call('hello.near-examples.testnet', 'set_greeting', { greeting: newGreeting });
  const updatedGreetingOnTestnet = await testnet.rootAccount.call('hello.near-examples.testnet', 'get_greeting', {});
  console.log('updatedGreetingOnTestnet:', updatedGreetingOnTestnet);

  t.is(updatedGreetingOnTestnet, newGreeting);
});

test('spoon contract', async (t) => {
  const { root } = t.context.accounts;
  const blockHeight = 186705486;

  const importedContract = await root.importContract({
    testnetContract: 'hello.near-examples.testnet',
    blockId: blockHeight,
  });

  const greeting = await importedContract.view('get_greeting', {});
  console.log('greeting:', greeting);

  t.is(greeting, 'Hello');
});