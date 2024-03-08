import { Command } from 'commander';

import { saveAgentConfig } from '../shared/context';
import { CONTAINER, Dependencies } from '../shared/ioc';

const contextCmd = new Command('context');

contextCmd
  .command('make-agent-config')
  .description('Make an agent config')
  .option('-o --output <output-dir>', 'The output directory')
  .action(async (_, cmd) => {
    const opts = cmd.optsWithGlobals();
    const { ctx, network } = CONTAINER.get(Dependencies);

    await saveAgentConfig(
      network,
      ctx,
      opts.output && { contextPath: opts.output },
    );
  });

export { contextCmd };
