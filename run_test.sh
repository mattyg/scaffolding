#!/usr/bin/bash
set -e

rm -rf /tmp/forum-svelte

cd /tmp

hc-scaffold web-app forum-svelte --setup-nix true --template svelte

cd forum-svelte

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type likes --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Likes,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection global all_likes likes
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash

hc-scaffold link-type post likes --delete true --bidireccional false
hc-scaffold link-type comment likes:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash likes --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true


nix develop --command bash -c "
set -e
npm i
npm run build -w ui
npm t
"

rm -rf /tmp/forum-vue
cd /tmp

hc-scaffold web-app forum-vue --setup-nix true --template vue
cd forum-vue

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type likes --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Likes,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_likes likes
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash

hc-scaffold link-type post likes --delete true --bidireccional false
hc-scaffold link-type comment likes:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash likes --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true

nix develop --command bash -c "
set -e
npm i
npm run build -w ui
npm t
"

rm -rf /tmp/forum-lit
cd /tmp

hc-scaffold web-app forum-lit --setup-nix true --template lit
cd forum-lit

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type likes --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Likes,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash
hc-scaffold collection global all_likes likes

hc-scaffold link-type post likes --delete true --bidireccional false
hc-scaffold link-type comment likes:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash likes --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true

nix develop --command bash -c "
set -e
npm i
npm run build -w ui
npm run format -w ui
npm run lint -w ui
npm t
"


rm -rf /tmp/forum-vanilla
cd /tmp

hc-scaffold web-app forum-vanilla --setup-nix true --template vanilla
cd forum-vanilla

hc-scaffold dna forum 
hc-scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/
hc-scaffold entry-type post --reference-entry-hash false --crud crud --link-from-original-to-each-update true --fields title:String:TextField,content:String:TextArea
hc-scaffold entry-type comment --reference-entry-hash false --crud crud --link-from-original-to-each-update false --fields post_hash:ActionHash::Post
hc-scaffold entry-type likes --reference-entry-hash false --crud crd --fields like_hash:Option\<ActionHash\>::Likes,string_list:Vec\<String\>
hc-scaffold entry-type certificate --reference-entry-hash true --crud cr --fields post_hash:ActionHash::Post,agent:AgentPubKey::certified,certifications_hashes:Vec\<EntryHash\>::Certificate,certificate_type:Enum::CertificateType:TypeOne.TypeTwo,dna_hash:DnaHash

hc-scaffold collection global all_posts post 
hc-scaffold collection by-author posts_by_author post
hc-scaffold collection global all_posts_entry_hash post:EntryHash
hc-scaffold collection global all_likes likes
hc-scaffold collection by-author posts_by_author_entry_hash post:EntryHash

hc-scaffold link-type post likes --delete true --bidireccional false
hc-scaffold link-type comment likes:EntryHash --delete true --bidireccional true
hc-scaffold link-type certificate:EntryHash likes --delete false --bidireccional false
hc-scaffold link-type agent:creator post:EntryHash --delete false --bidireccional true


nix develop --command bash -c "
set -e
npm i
npm t
npm run package
"
