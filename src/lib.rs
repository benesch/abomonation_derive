extern crate syn;
#[macro_use]
extern crate synstructure;
#[macro_use]
extern crate quote;

decl_derive!([Abomonation, attributes(unsafe_abomonate_ignore)] => derive_abomonation);

fn derive_abomonation(mut s: synstructure::Structure) -> quote::Tokens {
    s.filter(|bi| !bi.ast().attrs.iter().any(|attr| attr.name() == "unsafe_abomonate_ignore"));
    
    let entomb = s.each(|bi| quote! {
        ::abomonation::Abomonation::entomb(#bi, _write)?;
    });

    let extent = s.each(|bi| quote! {
        sum += ::abomonation::Abomonation::extent(#bi);
    });

    s.bind_with(|_| synstructure::BindStyle::RefMut);

    let exhume = s.each(|bi| quote! {
        let temp = bytes;
        bytes = ::abomonation::Abomonation::exhume(#bi, temp)?;
    });

    s.bound_impl("::abomonation::Abomonation", quote! {
        #[inline] unsafe fn entomb<W: ::std::io::Write>(&self, _write: &mut W) -> ::std::io::Result<()> {
            match *self { #entomb }
            Ok(())
        }
        #[allow(unused_mut)]
        #[inline] fn extent(&self) -> usize {
            let mut sum = 0;
            match *self { #extent }
            sum
        }
        #[allow(unused_mut)]
        #[inline] unsafe fn exhume<'a,'b>(
            &'a mut self,
            mut bytes: &'b mut [u8]
        ) -> Option<&'b mut [u8]> {
            match *self { #exhume }
            Some(bytes)
        }
    })
}