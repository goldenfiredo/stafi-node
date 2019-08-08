use stafi_primitives::AuraId;
use stafi_primitives::AccountId;
use grandpa::AuthorityId as GrandpaId;
use primitives::crypto::UncheckedInto;
use hex_literal::hex;

pub fn get_vals() -> Vec<(AccountId, AccountId, AuraId, GrandpaId)> {
	return vec![(
		// 5FWczK2Dsy8GfJxZFLdVxLDupcmuUUMCQnHR8uasMfNjL2Kg
		hex!["9878b1658ea4b8ac5793a1a46849b79405fb7ad8a587e3b368f88715cc0f9417"].unchecked_into(),
		// 5FkQamQ26G3RQoagZvtMr9HCoxbqbZMCxhpBeRg1Y6BUpzSV
		hex!["a2fc5dd0334f2b403806fe3a9cdd910021895ff8870179bc955a6d4d84c6da28"].unchecked_into(),
		// 5DGceEDy5fiGcgTetauVtx89UfuzoYNaJ4kFJpjhYjevmhcS
		hex!["35519ff28fe9b61b55ed9d26c2cb2355eb5c0025c7c0fa68d0b4ab94032e239d"].unchecked_into(),
		// 5GJZ7XteaotgoMRcbtMARFmwAEyfDaCYyUraZ3L7ytYi6eH5
		hex!["bb80ef7db77c1d188c991985b795a3105da52dd480310e205675092d47692afe"].unchecked_into(),
	
	), (
		// 5F7YNBnBNjhbgJf6pX93T5hwoxp7zwVUzgJWk6ZhiBBBCJ53
		hex!["86de7c9da69fa4ad3962976d6e79f90aa4c94089fcd1006511664b23c3a7a865"].unchecked_into(),
		// 5Cakgv7ESRrNJYnnTEMrZyz8my41K8RaqdaU9sC6SjPjpYgQ
		hex!["16eae51f7a0940c46226a5e7d7ae5788c03b597183825394a033231182adc217"].unchecked_into(),
		// 5Gk5rnC2SbLtdHf1xUaSBLt8EMk9ysTZVQFuKCkwAvQUDKGa
		hex!["cef991d3cac54a59312b0749b0c36c39edafa29d27f8aecd34656ee87d5be3ff"].unchecked_into(),
		// 5Dx6XQ6SGRwb6eUPa2xhwYJJtL4x78Gf81n7QPvry7kfbkhG
		hex!["536e0e399aaf6fbcd40130838bf466403b7fcd87e7af28c8c0f27728f61681f5"].unchecked_into(),

	)];
}

pub fn get_more_endowed() -> Vec<AccountId> {
	return vec![
		// 5CS79WV9TH3uQtGEKB9L3EWqT4pwYpMmhkJ1HjuvqaukzDwV
		hex!["10528e49509e0bdc84f442052a9ea4018f51240baffa7d35008023db9afea809"].unchecked_into(),
		// 5Esib4k7gpCfodaJfxsMmNrMWDJstJc4We8MU9rXGdyMC9w3
		hex!["7c53762b1da0cca7511f566af1e2b1dd9ee96c143834d94226d077e842bcdb1b"].unchecked_into(),
		// 5GU2voBubjcdrpM4HRcLWkiJyu953DMDvbmnQRww4nkb2XVR
		hex!["c2bbc7055688843bdfa5d67c3b1e406404dc395af7f84e4d4a6fbea695722f72"].unchecked_into(),
		// 5FkHzSvAiygEM5srvP6uT8ocujGc9zrxmBP7UUeT1HxqqdKZ
		hex!["a2e62d43d93d9fb11beac2915eda60ab73986a8ff81039b670306d72455b6445"].unchecked_into(),
	];
}

pub fn get_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5DhMdxA5Zw2LdCYJQSKrqmqZPrN438DJLA5BFWSjtt6DWUWr
		hex!["48303fd3c0d044cd17fd8281c5ffe8822864c5b133c3c11465e270bdccb76759"].unchecked_into(),
		// 5CfSUVug9dtgbHDguqqFsouZpdpUDGkCypgRHVBCEW7CCo4B
		hex!["1a7dced52d5566c5d59b67f0389a8f5ee2b4c2fa46de8bac4dd815d93dd6da0a"].unchecked_into(),
		// 5DtRLJV7DcX2JzzmGNWEYWMjq22oncgDHGYEH5f1az5f1LDy
		hex!["50a063bce8ed4b19c8fe0b1e77338381c80f9ab1f4b3bdb7a217fb4121bbf749"].unchecked_into(), 
		// 5Ck1a7t2UZy9qctr484Pb4w5TP5rVMrWFh3y6qnsEV9MS5xU
		hex!["1dfa3422b6b734142c756f43abd5c8be3c5c549967f008c0e738fa332310dad4"].unchecked_into(),
	];
}

pub fn get_root_key() -> AccountId {
	// 5F1Zz5KJWPPiA2UjrNpgeT6cd99Joa3hhf2pcMBGmxVKdAao
	return hex!["825076af6cf69ba92681b4deb8a0ad42b0113df7f6abeb85b507786c4f4db80b"].unchecked_into();
}

