// import 'dotenv/config';

// if (!process.env.HYPIXEL_API_KEY) {
//   throw new Error('HYPIXEL_API_KEY not set');
// }

// async function test(uuid: string) {
//   try {
//     const url = `https://api.hypixel.net/v2/skyblock/profiles?uuid=${uuid}`;
//     const response = await fetch(url, {
//       method: 'GET',
//       headers: {
//         'API-Key': process.env.HYPIXEL_API_KEY as string,
//       },
//     });
//     const data = await response.json();
//     const profiles = data.profiles;
//     let selectedProfile = null;
//     for (const profile of Object.values(profiles)) {
//       if (profile.selected) {
//         selectedProfile = profile;
//         break;
//       }
//     }
//     console.log(selectedProfile.game_mode);

//     if (selectedProfile && selectedProfile.members) {
//       let matchedMember = null;

//       // Iterate over members and find the matching UUID
//       Object.keys(selectedProfile.members).forEach((member) => {
//         if (member === uuid) {
//           matchedMember = selectedProfile.members[member];
//         }
//       });

//       console.log(matchedMember.currencies.coin_purse);
//     } else {
//       console.log('No selected profile or members found.');
//     }
//   } catch (error) {
//     console.log(error);
//   }
// }

// const uuid = '27a9fd8dcd8b4beca3753c2e318f44f1';
// test(uuid);
