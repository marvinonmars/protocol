import {LCDClient, MnemonicKey} from "@terra-money/terra.js";

export function initialize() {
  const mk = new MnemonicKey();

  console.log(`Account Address: ${mk.accAddress}`);
  console.log(`MnemonicKey: ${mk.mnemonic}`);

  return terra.wallet(mk);
}

export function recover(mnemonic) {
  const mk = new MnemonicKey({mnemonic: mnemonic});
  console.log(mk.accAddress);
  return terra.wallet(mk);
}

const terra = new LCDClient({
  URL: 'https://tequila-lcd.terra.dev',
  chainID: 'tequila-0004'
});

const wallet = initialize();
// const wallet = await recover(process.env.TEST_MAIN);

