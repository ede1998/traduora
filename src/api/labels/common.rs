use serde::Deserialize;

impl_wrapper!(LabelId, "Type-safe label id wrapper");

/// Default model.
///
/// **Endpoint**
/// - `POST /api/v1/projects/{projectId}/labels`
/// - `GET /api/v1/projects/{projectId}/labels`
/// - `PATCH /api/v1/projects/{projectId}/labels/{labelId}`
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    /// Unique id of the label.
    pub id: LabelId,
    /// The label display name.
    pub value: String,
    /// The color for this label. Usually in the hex form, e.g. `#D81159`.
    pub color: String,
}
