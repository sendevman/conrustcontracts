use concordium_cis1::*;
use concordium_std::*;

type TokenId = TokenIdU8;
type TokenCount = u64;

#[derive(Serial)]
enum Keys {
    SimpleMap,
    TokenState,
    MintEventCounter,
}

#[derive(Serialize, SchemaType)]
struct MintParams {
    owner:       Address,
    token_id:    TokenId,
    token_count: TokenCount,
}

// With GATs (Generic Associated Types), we could avoid the _ parameter and do:
// init<State: HasContractStateHL> .. {
//    let simple_map: State::Map<u8,u8> = state.new_map();
// }

#[init(contract = "new-state", parameter = "MintParams")]
fn init(_ctx: &impl HasInitContext, state: &mut impl HasContractStateHL) -> InitResult<()> {
    let simple_map: StateMap<_, u8, u8> = state.new_map();
    state.insert(Keys::SimpleMap, simple_map);

    let token_state: StateMap<_, TokenId, StateMap<_, TokenId, TokenCount>> = state.new_map();
    state.insert(Keys::TokenState, token_state);
    state.insert(Keys::MintEventCounter, 0u64);
    Ok(())
}

#[receive(contract = "new-state", name = "mint", parameter = "MintParams")]
fn receive<A: HasActions, State: HasContractStateHL>(
    ctx: &impl HasReceiveContext,
    _state: &mut State,
) -> ReceiveResult<A> {
    let params: MintParams = ctx.parameter_cursor().get()?;

    // Makes no changes in the state.
    // let mut simple_map = state.get(Keys::SimpleMap).unwrap_abort()?;

    // // Inserts value in state
    // let _ = simple_map.insert(0, 0);
    // // Inserts value in state
    // let _ = simple_map.insert(1, 2);

    // let mut token_state: StateMap<Address, TokenId> =
    //     state.get_map(Keys::TokenState).unwrap_abort()?;
    // let _ = token_state.insert(params.owner, params.token_id);

    //     token_state
    //         .entry(params.owner)
    //         .and_modify(|owner_map| {
    //             owner_map
    //                 .entry(params.token_id)
    //                 .and_modify(|current_count| *current_count +=
    // params.token_count)                 .or_insert(params.token_count);
    //         })
    //         .or_insert({
    //             let mut owner_map = state.new_map(); // Creating a nested map
    // without knowing its location. Only possible with the scoped entry API.
    //             let _ = owner_map.insert(params.token_id, params.token_count);
    //             owner_map
    //         });
    //     token_state
    //         .entry(params.owner)
    //         .and_modify(|owner_map| {
    //             (
    //                 counter + 1,
    //                 owner_map
    //                     .entry(params.token_id)
    //                     .and_modify(|current_count| *current_count +=
    // params.token_count)                     .or_insert(params.token_count),
    //             )
    //         })
    //         .or_insert({
    //             let mut owner_map = state.new_map();
    //              let _ = owner_map.insert(params.token_id,
    // params.token_count);             (0, owner_map)
    //         });

    Ok(A::accept())
}
