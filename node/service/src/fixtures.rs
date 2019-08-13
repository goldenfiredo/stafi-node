use stafi_primitives::AccountId;
use grandpa_primitives::{AuthorityId as GrandpaId};
use babe_primitives::{AuthorityId as BabeId};
use im_online::AuthorityId as ImOnlineId;
use primitives::crypto::UncheckedInto;
use hex_literal::hex;

pub fn get_vals() -> Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId)> {
	return vec![(
		// 5H3CkhEYS8nPAk5tr6VNk2SK5xG7LqAGxdazSn3Wr2f9tvww
		hex!["dc07f3d172eba4e007422ae84c822843de792eeb30f0f96cc91770cbb3814708"].unchecked_into(),
		// 5H1KEEffEk6AH2whhuqaKZLsFJDiroak1smMWZL2hBuAEccA
		hex!["da974207cfa42441fc5f33d7ea7591ae563c5c41948540d089e8bd09d6a5f77b"].unchecked_into(),
		// 5H4zsfCFFiCzvuSM7mN8oZDgPyUUMr6ssHPeByxyV7h8pczV
		hex!["dd6673ad0a0989e79bca2e9e4283c2950f55bbe5e81cd336b71509e092504aa1"].unchecked_into(),
		// 5CFhFuw93dEptMYx9ydn4JgvrH51pvgXZiK9JqubbKST6y4j
		hex!["0861aea8c80cd621db3f227ee20f44ed02e5e2a00db7d1445c94d19ce61823e9"].unchecked_into(),
		// 5Ew99iKaTSpttYKqD2UsYzjhAwpshMKd8FAvRkkzLrPfYbfi
		hex!["7eefe24615aa73ec442a8171f5d5df6324dcceb5ba2ae25737da8ee30a909204"].unchecked_into(),
	
	), (
		// 5F4pU68yZLaUSJhJL5jyuZXGLK7hN3w6GmCkztTPg3fz2jad
		hex!["84caf436cf130a556875105eeb153fb3df812bb80f1b5414088f9a980e76886a"].unchecked_into(),
		// 5G3d1sfoU3CgnEjc13vBrYSUQ6aN6HBiDysM15hvKUza7Ss6
		hex!["b01d6425bfa1f942460764d8fc8b1242617f5fd6f6d55344eaa7d0a352ecb03e"].unchecked_into(),
		// 5DDaeMPyKFtQGDQtH2ZkkEJdErQSJkTYqwxpuMcQEr91wyeN
		hex!["330129c4cb93ab7b8c59087033614172d5ff749fc9db0afb647e4e621616ec37"].unchecked_into(),
		// 5H91LFqkGF2BSLsEhnu95swx8waWr6nM67XBUu5VRvhvRUWe
		hex!["e074fa2854512385922cfab97b8c1ffd92e255ead0a8a06784f30cff8d4567dc"].unchecked_into(),
		// 5GDtbpjak9xyfpymw5KJZb1iYxxG1ihmnVfSyDtFXqLMv3HA
		hex!["b7f24f667700924e9a06af03fc4ac201ea51727428f84108db0dd49ac4ffc616"].unchecked_into(),

	)];
}

pub fn get_more_endowed() -> Vec<AccountId> {
	return vec![
		// 5FQnt5yLPr9ALgQ6ePtV55b6141ohK1jiY57jrBdhL1WxPFV
		hex!["940685b11da604bf618d39aaeea56e9b8d68b0fbaffeba59e9018c7e439f5b45"].unchecked_into(),
		// 5D7R4eNTu31joPF9k3FHEx4eotusmBrYSuChs5HDnkL6wD3T
		hex!["2e4d700b1cbb2e3ecfbacf5051597445cabb8ba6fb8f665f8987a828e4bbca7c"].unchecked_into(),
		// 5HBJGF3NHhRyzPD3sqgXQDZwr8eMr94nvoZe6VnAqUJghxuh
		hex!["e234764e173d1320f8bd56ee09aaeeb543b55b38957c6d7eb2caaee8c041888c"].unchecked_into(),
		// 5Fs3kySyDPG6G14XMiYRHYokdhFJ25W8vYCPHV2mQDxgTqEX
		hex!["a80cfc0470dcc30dcda4dfe7ac89c51ac691c5572c7dc04da41687194f15e698"].unchecked_into(),
		// 5FzCb8Srxd5G9FywBeL38Gjaa3fwzjZJgS81pBzRCTdT6Tc5
		hex!["ad816d75fe87a5fbd875362c223a54a428051301049f21259ffb65d9ca9ed845"].unchecked_into(),
	];
}

pub fn get_root_key() -> AccountId {
	// 5Hgp2W18KV4ZTaQSqpU73amsiSwUpzzYbbXgKk4MTfQKj1WJ
	return hex!["f8b6c530b0f898a992089f43b48357adfce8038b535ba4cfa805802d6e35c720"].unchecked_into();
}

