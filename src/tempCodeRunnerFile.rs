hash_elem_impl!(SignatureMessage, |data| {
    SignatureMessage(hash_to_fr(data))
});