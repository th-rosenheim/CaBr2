export interface SubstanceData {
  name: Data<string>;
  readonly alternativeNames: string[];
  cas: Data<string | undefined>;
  molecularFormula: Data<string>;
  molarMass: Data<string | undefined>;
  meltingPoint: Data<string | undefined>;
  boilingPoint: Data<string | undefined>;
  waterHazardClass: Data<string | undefined>;
  hPhrases: Data<[string, string][]>;
  pPhrases: Data<[string, string][]>;
  signalWord: Data<string | undefined>;
  symbols: Data<string[]>;
  lethalDose: Data<string | undefined>;
  mak: Data<string | undefined>;
  readonly source: Source;
  amount: Data<Amount | undefined>;
}

export interface Data<T> {
  modifiedData?: T;
  readonly originalData: T;
}

export interface Image {
  src: string;
  alt: string;
}

export interface Source {
  provider: string;
  url: string;
  lastUpdated: Date;
}

export interface Amount {
  value: string;
  unit: Unit;
}

export enum Unit {
  LITRE = 'LITRE',
  MILLILITER = 'MILLILITER',
  MICROLITRE = 'MICROLITRE',
  GRAM = 'GRAM',
  MILLIGRAM = 'MILLIGRAM',
  MICROGRAM = 'MICROGRAM',
  PIECES = 'PIECES',
  CUSTOM = 'CUSTOM', // needs String
}

export enum TemperatureUnit {
  CELSIUS = 'CELSIUS',
  FAHRENHEIT = 'FAHRENHEIT',
}

export interface UnitMapping<T> {
  viewValue: string;
  value: T;
}

const unitMappings: UnitMapping<Unit>[] = [
  { viewValue: 'l (Liter)', value: Unit.LITRE },
  { viewValue: 'ml (Milliliter)', value: Unit.MILLILITER },
  { viewValue: 'µl (Mikroliter)', value: Unit.MICROLITRE },
  { viewValue: 'g (Gramm)', value: Unit.GRAM },
  { viewValue: 'mg (Milligramm)', value: Unit.MILLIGRAM },
  { viewValue: 'µg (Mikrogramm)', value: Unit.MICROGRAM },
  { viewValue: 'Stück', value: Unit.PIECES },
  { viewValue: 'Custom', value: Unit.CUSTOM }, // TODO implement custom type
];

const temperatureUnitMapping: UnitMapping<TemperatureUnit>[] = [
  { viewValue: '°C', value: TemperatureUnit.CELSIUS },
  { viewValue: 'F', value: TemperatureUnit.FAHRENHEIT },
];

export { unitMappings, temperatureUnitMapping };
