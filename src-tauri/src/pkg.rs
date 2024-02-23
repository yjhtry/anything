use pkg_service::{
    types::{
        Package, PackageAddReq, PackageAddRes, PackageCategoryAddReq, PackageCategoryAddRes,
        PackageCategoryQueryReq, PackageCategoryQueryRes, PackageCategoryUpdateReq,
        PackageCategoryUpdateRes, PackageQueryReq, PackageQueryRes, PackageUpdateReq,
        PackageUpdateRes,
    },
    PackManager, Pkg, PkgError,
};
use tauri::State;

#[tauri::command]
pub async fn query_packages(
    data: PackageQueryReq,
    state: State<'_, PackManager>,
) -> Result<PackageQueryRes, PkgError> {
    let pack_manager = state.inner();

    pack_manager.query_packages(data).await
}

#[tauri::command]
pub async fn add_package(
    data: PackageAddReq,
    state: State<'_, PackManager>,
) -> Result<PackageAddRes, PkgError> {
    let pack_manager = state.inner();

    pack_manager.add_package(data).await
}

#[tauri::command]
pub async fn update_package(
    data: PackageUpdateReq,
    state: State<'_, PackManager>,
) -> Result<PackageUpdateRes, PkgError> {
    let pack_manager = state.inner();

    pack_manager.update_package(data).await
}

#[tauri::command]
pub async fn delete_package(id: i64, state: State<'_, PackManager>) -> Result<(), PkgError> {
    let pack_manager = state.inner();

    pack_manager.delete_package(id).await
}

#[tauri::command]
pub async fn get_package_by_id(
    id: i64,
    state: State<'_, PackManager>,
) -> Result<Package, PkgError> {
    let pack_manager = state.inner();

    pack_manager.get_package_by_id(id).await
}

#[tauri::command]
pub async fn add_category(
    data: PackageCategoryAddReq,
    state: State<'_, PackManager>,
) -> Result<PackageCategoryAddRes, PkgError> {
    let pack_manager = state.inner();

    pack_manager.add_category(data).await
}

#[tauri::command]
pub async fn query_categories(
    data: PackageCategoryQueryReq,
    state: State<'_, PackManager>,
) -> Result<PackageCategoryQueryRes, PkgError> {
    let pack_manager = state.inner();

    pack_manager.query_categories(data).await
}

#[tauri::command]
pub async fn delete_category(id: i64, state: State<'_, PackManager>) -> Result<(), PkgError> {
    let pack_manager = state.inner();

    pack_manager.delete_category(id).await
}

#[tauri::command]
pub async fn update_category(
    data: PackageCategoryUpdateReq,
    state: State<'_, PackManager>,
) -> Result<PackageCategoryUpdateRes, PkgError> {
    let pack_manager = state.inner();

    pack_manager.update_category(data).await
}
