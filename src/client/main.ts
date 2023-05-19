/**
 * Hello world
 */
import minimist from 'minimist';

import {
  establishConnection,
  establishPayer,
  checkProgram,
  sayHello,
  reportGreetings,
} from './hello_world';

async function main() {
  const argv = minimist(process.argv.slice(2));

  console.log("Let's say hello to a Domichain account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Say hello to an account
  await sayHello(argv.compute_units);

  // Find out how many times that account has been greeted
  await reportGreetings();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
