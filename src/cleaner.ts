import type {Row} from './types';
export interface CleaningReport {files:Row[];verified_deleted_bytes:number;volume_free_before:number|null;volume_free_after:number|null;volume_free_delta:number|null;measurement_note:string;reversible:false}
export function cleaningOutcome(report:CleaningReport){const removed=report.files.filter(f=>f.deleted===true&&f.verified===true);const failed=report.files.length-removed.length;return {removed:removed.length,failed,bytes:report.verified_deleted_bytes,partial:failed>0};}
