use icicle_cuda_runtime::memory::HostOrDeviceSlice;

use crate::{error::IcicleResult, traits::FieldImpl};

pub trait Virgo<F: FieldImpl> {
    fn bk_sum_all_case_1(
        table1: &HostOrDeviceSlice<F>,
        table2: &HostOrDeviceSlice<F>,
        result: &mut HostOrDeviceSlice<F>,
        n: u32,
    ) -> IcicleResult<()>;

    fn bk_sum_all_case_2(table: &HostOrDeviceSlice<F>, result: &mut HostOrDeviceSlice<F>, n: u32) -> IcicleResult<()>;

    fn bk_produce_case_1(
        table1: &HostOrDeviceSlice<F>,
        table2: &HostOrDeviceSlice<F>,
        result: &mut HostOrDeviceSlice<F>,
        n: u32,
    ) -> IcicleResult<()>;
}

pub fn bk_sum_all_case_1<F>(
    table1: &HostOrDeviceSlice<F>,
    table2: &HostOrDeviceSlice<F>,
    result: &mut HostOrDeviceSlice<F>,
    n: u32,
) -> IcicleResult<()>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: Virgo<F>,
{
    <<F as FieldImpl>::Config as Virgo<F>>::bk_sum_all_case_1(table1, table2, result, n)
}

pub fn bk_sum_all_case_2<F>(table: &HostOrDeviceSlice<F>, result: &mut HostOrDeviceSlice<F>, n: u32) -> IcicleResult<()>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: Virgo<F>,
{
    <<F as FieldImpl>::Config as Virgo<F>>::bk_sum_all_case_2(table, result, n)
}

pub fn bk_produce_case_1<F>(
    table1: &HostOrDeviceSlice<F>,
    table2: &HostOrDeviceSlice<F>,
    result: &mut HostOrDeviceSlice<F>,
    n: u32,
) -> IcicleResult<()>
where
    F: FieldImpl,
    <F as FieldImpl>::Config: Virgo<F>,
{
    <<F as FieldImpl>::Config as Virgo<F>>::bk_produce_case_1(table1, table2, result, n)
}

#[macro_export]
macro_rules! impl_virgo {
    (
        $field_prefix:literal,
        $field_prefix_ident:ident,
        $field:ident,
        $field_config:ident
      ) => {
        mod $field_prefix_ident {
            use crate::virgo::{$field, $field_config, CudaError, DeviceContext};

            extern "C" {
                #[link_name = concat!($field_prefix, "BkSumAllCase1")]
                pub(crate) fn _bk_sum_all_case_1(
                    table1: *const $field,
                    table2: *const $field,
                    result: *mut $field,
                    n: u32,
                ) -> CudaError;
            }

            extern "C" {
                #[link_name = concat!($field_prefix, "BkSumAllCase2")]
                pub(crate) fn _bk_sum_all_case_2(arr: *const $field, result: *mut $field, n: u32) -> CudaError;
            }

            extern "C" {
                #[link_name = concat!($field_prefix, "BkProduceCase1")]
                pub(crate) fn _bk_produce_case_1(
                    table1: *const $field,
                    table2: *const $field,
                    result: *mut $field,
                    n: u32,
                ) -> CudaError;
            }
        }

        impl Virgo<$field> for $field_config {
            fn bk_sum_all_case_1(
                table1: &HostOrDeviceSlice<$field>,
                table2: &HostOrDeviceSlice<$field>,
                result: &mut HostOrDeviceSlice<$field>,
                n: u32,
            ) -> IcicleResult<()> {
                unsafe {
                    $field_prefix_ident::_bk_sum_all_case_1(table1.as_ptr(), table2.as_ptr(), result.as_mut_ptr(), n)
                        .wrap()
                }
            }

            fn bk_sum_all_case_2(
                table: &HostOrDeviceSlice<$field>,
                result: &mut HostOrDeviceSlice<$field>,
                n: u32,
            ) -> IcicleResult<()> {
                unsafe { $field_prefix_ident::_bk_sum_all_case_2(table.as_ptr(), result.as_mut_ptr(), n).wrap() }
            }

            fn bk_produce_case_1(
                table1: &HostOrDeviceSlice<$field>,
                table2: &HostOrDeviceSlice<$field>,
                result: &mut HostOrDeviceSlice<$field>,
                n: u32,
            ) -> IcicleResult<()> {
                unsafe {
                    $field_prefix_ident::_bk_produce_case_1(table1.as_ptr(), table2.as_ptr(), result.as_mut_ptr(), n)
                        .wrap()
                }
            }
        }
    };
}
