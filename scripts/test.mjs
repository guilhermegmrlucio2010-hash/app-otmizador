import {startVitest} from 'vitest/node';
const context=await startVitest('test',[],{run:true,config:false,include:['src/**/*.test.{ts,tsx}'],pool:'forks',maxWorkers:1,minWorkers:1});
if(!context)process.exitCode=1;
else {process.exitCode=context.state.getFiles().some(file=>file.result?.state==='fail')?1:0;await context.close();}
