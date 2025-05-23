use api_v1::models::AdCreateObject;
use common::mkpl_ad::*;
use common::*;

use crate::mappers::base::{
    AdTypeConverter, ProductIdConverter, StringFieldConverter, VisibilityConverter,
};

#[derive(Debug)]
pub struct AdCreateObjectMapper;

impl AdCreateObjectMapper {
    pub fn from_api(value: &Option<AdCreateObject>) -> MkplAd {
        match value {
            Some(api_obj) => MkplAd {
                id: MkplAdId::none(),
                title: StringFieldConverter::from_api(&api_obj.title),
                description: StringFieldConverter::from_api(&api_obj.description),
                ad_type: AdTypeConverter::from_api(&api_obj.ad_type),
                visibility: VisibilityConverter::from_api(&api_obj.visibility),
                product_id: ProductIdConverter::from_api(&api_obj.product_id),
                lock: MkplAdLock::none(),
            },
            None => MkplAd::none(),
        }
    }

    pub fn to_api(mkpl_ad: &MkplAd) -> Option<AdCreateObject> {
        Some(AdCreateObject {
            title: StringFieldConverter::to_api(&mkpl_ad.title),
            description: StringFieldConverter::to_api(&mkpl_ad.description),
            ad_type: AdTypeConverter::to_api(&mkpl_ad.ad_type),
            visibility: VisibilityConverter::to_api(&mkpl_ad.visibility),
            product_id: ProductIdConverter::to_api(&mkpl_ad.product_id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stubs::StubsMkplAd;

    #[test]
    fn test_full_conversion_cycle() {
        let mkpl = StubsMkplAd::case1();
        let api = AdCreateObjectMapper::to_api(&mkpl).unwrap();

        assert_eq!(mkpl.title, *api.title.as_ref().unwrap());
        assert_eq!(mkpl.description, *api.description.as_ref().unwrap());
        // assert_eq!(mkpl.ad_type, api.ad_type.as_ref().unwrap());
        // assert_eq!(mkpl.visibility, api.visibility.as_ref().unwrap());
        assert_eq!(mkpl.product_id.get(), api.product_id.as_ref().unwrap());

        let converted_back = AdCreateObjectMapper::from_api(&Some(api));

        assert_eq!(MkplAdId::none(), converted_back.id);
        assert_eq!(mkpl.title, converted_back.title);
        assert_eq!(mkpl.description, converted_back.description);
        assert_eq!(mkpl.ad_type, converted_back.ad_type);
        assert_eq!(mkpl.visibility, converted_back.visibility);
        assert_eq!(MkplAdLock::none(), converted_back.lock);
        assert_eq!(mkpl.product_id, converted_back.product_id);
    }
}
