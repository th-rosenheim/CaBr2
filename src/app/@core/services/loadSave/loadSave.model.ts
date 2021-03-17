import { Header } from '../../interfaces/Header';
import { SubstanceData } from '../../models/substances.model';

export interface CaBr2Document {
  header: Header;
  substanceData: SubstanceData[];
  humanAndEnvironmentDanger: string[];
  rulesOfConduct: string[];
  inCaseOfDanger: string[];
  disposal: string[];
}

export interface DocumentTypes {
  load: string[];
  save: string[];
}
