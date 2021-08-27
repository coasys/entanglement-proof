import type { Address, Language, HolochainLanguageDelegate, LanguageContext, Interaction } from "@perspect3vism/ad4m";
import { DNA, DNA_NICK } from "./dna";
import Adapter from "./adapter";
import EntanglementProofAuthorAdapter from "./authorAdapter";

export const name = "entanglement-proof-store";

function interactions(expression: Address): Interaction[] {
  return [];
}

export default async function create(context: LanguageContext): Promise<Language> {
  const Holochain = context.Holochain as HolochainLanguageDelegate;
  await Holochain.registerDNAs([{ file: DNA, nick: DNA_NICK }]);

  const expressionAdapter = new Adapter(context);
  const getByAuthorAdapter = new EntanglementProofAuthorAdapter(context);

  return {
    name,
    expressionAdapter,
    getByAuthorAdapter,
    interactions,
  } as Language;
}
