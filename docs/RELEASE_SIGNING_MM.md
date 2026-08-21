# Zap Release Signing နှင့် Verification

## အကျုံးဝင်မှု

Zap release signing သည် ephemeral CI GPG keyring ကို အသုံးပြုပါသည်။ Repository ထဲတွင် scripts နှင့် policy များသာ ရှိပြီး private signing key သို့မဟုတ် passphrase မရှိပါ။ လိုအပ်သော private-key secret မရှိပါက release workflow သည် fail closed ဖြစ်ပါသည်။

## လိုအပ်သော Protected Secrets

Protected GitHub Actions release environment ထဲတွင် အောက်ပါတန်ဖိုးများကို သတ်မှတ်ရပါမည်။

| Secret | လိုအပ်မှု | ရည်ရွယ်ချက် |
|---|---:|---|
| `ZAP_RELEASE_GPG_PRIVATE_KEY` | Yes | Ephemeral runner keyring အတွင်းသာ အသုံးပြုမည့် ASCII-armored private key |
| `ZAP_RELEASE_GPG_PASSPHRASE` | Optional | Protected private key အတွက် passphrase; မထည့်ပါက ရည်ရွယ်ချက်ရှိရှိ unprotected CI key ဖြစ်ရမည် |

Private key ကို runner ၏ temporary `GNUPGHOME` ထဲသို့သာ import လုပ်ရမည်။ Commit မလုပ်ရ၊ example file ထဲ မထည့်ရ၊ logs ထဲ မထုတ်ရ၊ release asset အဖြစ် မပါဝင်ရပါ။

## Public Verification Artifact

Workflow သည် signing key ၏ public portion ကိုသာ အောက်ပါ command ဖြင့် export လုပ်ပါသည်။

```bash
GNUPGHOME="$GNUPGHOME" \
SIGNING_KEY_ID="$SIGNING_KEY_ID" \
  bash scripts/export_release_public_key.sh \
    "artifacts/zap-${GITHUB_REF_NAME#v}-release-signing-key.asc"
```

အသုံးပြုသူများနှင့် downstream automation များက `.asc` signatures များကို verify လုပ်နိုင်ရန် public key ကို release နှင့်အတူ ဖြန့်ဝေပါသည်။ Helper သည် output အလွတ်ဖြစ်ခြင်းကို reject လုပ်ပြီး private-key armor block ပါဝင်သော output ကိုလည်း မထုတ်ပြန်ပါ။

Machine-readable controls များကို `deploy/release-signing-policy.toml` တွင် သတ်မှတ်ထားပါသည်။

## Local Verification

Release တစ်ခုကို download လုပ်ပြီး trusted public key ကို isolated verification keyring ထဲသို့ import လုပ်ပြီးနောက် အောက်ပါ command ကို run ပါ။

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.0 ./published-release
```

Verifier သည် archive set၊ per-artifact checksums၊ aggregate checksums၊ manifest/provenance consistency၊ သတ်မှတ်ထားသော archive entries နှင့် detached signatures များကို စစ်ဆေးပါသည်။ Asset ပျောက်ဆုံးခြင်း၊ hash မကိုက်ခြင်း၊ signature ပျောက်ဆုံးခြင်း၊ unsafe name သို့မဟုတ် provenance မမှန်ခြင်းများတွင် fail closed ဖြစ်ပါသည်။

## Key Rotation

Key rotation ပြုလုပ်ရာတွင် key ID အသစ်၊ protected secret update၊ public-key distribution၊ signed fixture နှင့် release verification run အောင်မြင်မှု၊ bilingual release notice တို့ လိုအပ်ပါသည်။ Key အဟောင်းကို revoke လုပ်ပါက revocation နှင့် key အသစ်ဖြင့် sign လုပ်ထားသော ပထမဆုံး release ကို တစ်ပြိုင်နက် အသိပေးရပါမည်။ Security incident မဟုတ်ပါက ရှိပြီးသား releases များကို မူလ trusted key ဖြင့် ဆက်လက် verify လုပ်နိုင်ရပါမည်။

## Release Gates

Public release တစ်ခုအတွက် release preflight၊ deterministic artifact manifest၊ aggregate checksum၊ provenance၊ signatures၊ post-publish verification နှင့် protected release environment manual approval အားလုံး လိုအပ်ပါသည်။ Automatic tag creation၊ automatic secret rotation နှင့် private-key export တို့ကို policy အရ ပိတ်ထားပါသည်။

## လုပ်ငန်းလည်ပတ်မှု နယ်နိမိတ်

Secret provisioning၊ key custody၊ public-key trust distribution နှင့် key rotation တို့သည် operator တာဝန်များ ဖြစ်ပါသည်။ Repository သည် ပြန်လည်အသုံးပြုနိုင်သော procedure နှင့် validation scripts များကို ပေးသော်လည်း production credentials များ မပါဝင်သကဲ့သို့ production access ကိုလည်း တစ်ဦးတည်း ခွင့်ပြုထားခြင်း မရှိပါ။
