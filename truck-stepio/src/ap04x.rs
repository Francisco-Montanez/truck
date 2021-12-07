#![allow(dead_code)]
pub mod ap_04x {
    use crate::{
        as_holder, derive_more::*, error::Result, primitive::*, tables::*, Holder, TableInit,
    };
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Default, TableInit)]
    pub struct Tables {
        application_context: HashMap<u64, as_holder!(ApplicationContext)>,
        application_protocol_definition: HashMap<u64, as_holder!(ApplicationProtocolDefinition)>,
        application_context_element: HashMap<u64, as_holder!(ApplicationContextElement)>,
        product_context: HashMap<u64, as_holder!(ProductContext)>,
        product_definition_context: HashMap<u64, as_holder!(ProductDefinitionContext)>,
        product_concept_context: HashMap<u64, as_holder!(ProductConceptContext)>,
        library_context: HashMap<u64, as_holder!(LibraryContext)>,
        product: HashMap<u64, as_holder!(Product)>,
        product_category: HashMap<u64, as_holder!(ProductCategory)>,
        product_related_product_category: HashMap<u64, as_holder!(ProductRelatedProductCategory)>,
        product_category_relationship: HashMap<u64, as_holder!(ProductCategoryRelationship)>,
        product_definition_formation: HashMap<u64, as_holder!(ProductDefinitionFormation)>,
        product_definition_formation_relationship:
            HashMap<u64, as_holder!(ProductDefinitionFormationRelationship)>,
        product_definition_formation_with_specified_source:
            HashMap<u64, as_holder!(ProductDefinitionFormationWithSpecifiedSource)>,
        product_definition: HashMap<u64, as_holder!(ProductDefinition)>,
        product_definition_with_associated_documents:
            HashMap<u64, as_holder!(ProductDefinitionWithAssociatedDocuments)>,
        product_definition_relationship: HashMap<u64, as_holder!(ProductDefinitionRelationship)>,
        product_definition_substitute: HashMap<u64, as_holder!(ProductDefinitionSubstitute)>,
        product_definition_effectivity: HashMap<u64, as_holder!(ProductDefinitionEffectivity)>,
        characterized_object: HashMap<u64, as_holder!(CharacterizedObject)>,
        property_definition: HashMap<u64, as_holder!(PropertyDefinition)>,
        product_definition_shape: HashMap<u64, as_holder!(ProductDefinitionShape)>,
        shape_aspect: HashMap<u64, as_holder!(ShapeAspect)>,
        shape_aspect_relationship: HashMap<u64, as_holder!(ShapeAspectRelationship)>,
        shape_representation: HashMap<u64, as_holder!(ShapeRepresentation)>,
        property_definition_representation:
            HashMap<u64, as_holder!(PropertyDefinitionRepresentation)>,
        shape_representation_relationship:
            HashMap<u64, as_holder!(ShapeRepresentationRelationship)>,
        context_dependent_shape_representation:
            HashMap<u64, as_holder!(ContextDependentShapeRepresentation)>,
        shape_definition_representation: HashMap<u64, as_holder!(ShapeDefinitionRepresentation)>,
        name_assignment: HashMap<u64, as_holder!(NameAssignment)>,
        external_referent_assignment: HashMap<u64, as_holder!(ExternalReferentAssignment)>,
        library_assignment: HashMap<u64, as_holder!(LibraryAssignment)>,
        document_reference: HashMap<u64, as_holder!(DocumentReference)>,
        action_request_assignment: HashMap<u64, as_holder!(ActionRequestAssignment)>,
        action_assignment: HashMap<u64, as_holder!(ActionAssignment)>,
        certification_assignment: HashMap<u64, as_holder!(CertificationAssignment)>,
        approval_assignment: HashMap<u64, as_holder!(ApprovalAssignment)>,
        contract_assignment: HashMap<u64, as_holder!(ContractAssignment)>,
        security_classification_assignment:
            HashMap<u64, as_holder!(SecurityClassificationAssignment)>,
        person_assignment: HashMap<u64, as_holder!(PersonAssignment)>,
        organization_assignment: HashMap<u64, as_holder!(OrganizationAssignment)>,
        person_and_organization_assignment:
            HashMap<u64, as_holder!(PersonAndOrganizationAssignment)>,
        date_assignment: HashMap<u64, as_holder!(DateAssignment)>,
        time_assignment: HashMap<u64, as_holder!(TimeAssignment)>,
        date_and_time_assignment: HashMap<u64, as_holder!(DateAndTimeAssignment)>,
        group_assignment: HashMap<u64, as_holder!(GroupAssignment)>,
        effectivity_assignment: HashMap<u64, as_holder!(EffectivityAssignment)>,
        document_type: HashMap<u64, as_holder!(DocumentType)>,
        document: HashMap<u64, as_holder!(Document)>,
        document_with_class: HashMap<u64, as_holder!(DocumentWithClass)>,
        document_usage_constraint: HashMap<u64, as_holder!(DocumentUsageConstraint)>,
        document_relationship: HashMap<u64, as_holder!(DocumentRelationship)>,
        action: HashMap<u64, as_holder!(Action)>,
        executed_action: HashMap<u64, as_holder!(ExecutedAction)>,
        directed_action: HashMap<u64, as_holder!(DirectedAction)>,
        action_status: HashMap<u64, as_holder!(ActionStatus)>,
        action_request_status: HashMap<u64, as_holder!(ActionRequestStatus)>,
        action_relationship: HashMap<u64, as_holder!(ActionRelationship)>,
        action_method: HashMap<u64, as_holder!(ActionMethod)>,
        action_request_solution: HashMap<u64, as_holder!(ActionRequestSolution)>,
        action_method_relationship: HashMap<u64, as_holder!(ActionMethodRelationship)>,
        versioned_action_request: HashMap<u64, as_holder!(VersionedActionRequest)>,
        action_directive: HashMap<u64, as_holder!(ActionDirective)>,
        action_resource: HashMap<u64, as_holder!(ActionResource)>,
        action_resource_relationship: HashMap<u64, as_holder!(ActionResourceRelationship)>,
        action_resource_type: HashMap<u64, as_holder!(ActionResourceType)>,
        certification_type: HashMap<u64, as_holder!(CertificationType)>,
        certification: HashMap<u64, as_holder!(Certification)>,
        approval_status: HashMap<u64, as_holder!(ApprovalStatus)>,
        approval: HashMap<u64, as_holder!(Approval)>,
        approval_date_time: HashMap<u64, as_holder!(ApprovalDateTime)>,
        approval_person_organization: HashMap<u64, as_holder!(ApprovalPersonOrganization)>,
        approval_role: HashMap<u64, as_holder!(ApprovalRole)>,
        approval_relationship: HashMap<u64, as_holder!(ApprovalRelationship)>,
        security_classification_level: HashMap<u64, as_holder!(SecurityClassificationLevel)>,
        security_classification: HashMap<u64, as_holder!(SecurityClassification)>,
        contract_type: HashMap<u64, as_holder!(ContractType)>,
        contract: HashMap<u64, as_holder!(Contract)>,
        address: HashMap<u64, as_holder!(Address)>,
        personal_address: HashMap<u64, as_holder!(PersonalAddress)>,
        organizational_address: HashMap<u64, as_holder!(OrganizationalAddress)>,
        person: HashMap<u64, as_holder!(Person)>,
        organization: HashMap<u64, as_holder!(Organization)>,
        organizational_project: HashMap<u64, as_holder!(OrganizationalProject)>,
        person_and_organization: HashMap<u64, as_holder!(PersonAndOrganization)>,
        organization_relationship: HashMap<u64, as_holder!(OrganizationRelationship)>,
        person_and_organization_role: HashMap<u64, as_holder!(PersonAndOrganizationRole)>,
        person_role: HashMap<u64, as_holder!(PersonRole)>,
        organization_role: HashMap<u64, as_holder!(OrganizationRole)>,
        date: HashMap<u64, as_holder!(Date)>,
        calendar_date: HashMap<u64, as_holder!(CalendarDate)>,
        ordinal_date: HashMap<u64, as_holder!(OrdinalDate)>,
        week_of_year_and_day_date: HashMap<u64, as_holder!(WeekOfYearAndDayDate)>,
        coordinated_universal_time_offset: HashMap<u64, as_holder!(CoordinatedUniversalTimeOffset)>,
        local_time: HashMap<u64, as_holder!(LocalTime)>,
        date_and_time: HashMap<u64, as_holder!(DateAndTime)>,
        date_time_role: HashMap<u64, as_holder!(DateTimeRole)>,
        date_role: HashMap<u64, as_holder!(DateRole)>,
        time_role: HashMap<u64, as_holder!(TimeRole)>,
        group: HashMap<u64, as_holder!(Group)>,
        group_relationship: HashMap<u64, as_holder!(GroupRelationship)>,
        effectivity: HashMap<u64, as_holder!(Effectivity)>,
        serial_numbered_effectivity: HashMap<u64, as_holder!(SerialNumberedEffectivity)>,
        dated_effectivity: HashMap<u64, as_holder!(DatedEffectivity)>,
        lot_effectivity: HashMap<u64, as_holder!(LotEffectivity)>,
        external_source: HashMap<u64, as_holder!(ExternalSource)>,
        external_source_relationship: HashMap<u64, as_holder!(ExternalSourceRelationship)>,
        pre_defined_item: HashMap<u64, as_holder!(PreDefinedItem)>,
        externally_defined_item: HashMap<u64, as_holder!(ExternallyDefinedItem)>,
        named_unit: HashMap<u64, as_holder!(NamedUnit)>,
        si_unit: HashMap<u64, as_holder!(SiUnit)>,
        conversion_based_unit: HashMap<u64, as_holder!(ConversionBasedUnit)>,
        context_dependent_unit: HashMap<u64, as_holder!(ContextDependentUnit)>,
        length_unit: HashMap<u64, as_holder!(LengthUnit)>,
        mass_unit: HashMap<u64, as_holder!(MassUnit)>,
        time_unit: HashMap<u64, as_holder!(TimeUnit)>,
        electric_current_unit: HashMap<u64, as_holder!(ElectricCurrentUnit)>,
        thermodynamic_temperature_unit: HashMap<u64, as_holder!(ThermodynamicTemperatureUnit)>,
        amount_of_substance_unit: HashMap<u64, as_holder!(AmountOfSubstanceUnit)>,
        luminous_intensity_unit: HashMap<u64, as_holder!(LuminousIntensityUnit)>,
        plane_angle_unit: HashMap<u64, as_holder!(PlaneAngleUnit)>,
        solid_angle_unit: HashMap<u64, as_holder!(SolidAngleUnit)>,
        area_unit: HashMap<u64, as_holder!(AreaUnit)>,
        volume_unit: HashMap<u64, as_holder!(VolumeUnit)>,
        ratio_unit: HashMap<u64, as_holder!(RatioUnit)>,
        dimensional_exponents: HashMap<u64, as_holder!(DimensionalExponents)>,
        derived_unit_element: HashMap<u64, as_holder!(DerivedUnitElement)>,
        derived_unit: HashMap<u64, as_holder!(DerivedUnit)>,
        global_unit_assigned_context: HashMap<u64, as_holder!(GlobalUnitAssignedContext)>,
        measure_with_unit: HashMap<u64, as_holder!(MeasureWithUnit)>,
        length_measure_with_unit: HashMap<u64, as_holder!(LengthMeasureWithUnit)>,
        mass_measure_with_unit: HashMap<u64, as_holder!(MassMeasureWithUnit)>,
        time_measure_with_unit: HashMap<u64, as_holder!(TimeMeasureWithUnit)>,
        electric_current_measure_with_unit:
            HashMap<u64, as_holder!(ElectricCurrentMeasureWithUnit)>,
        thermodynamic_temperature_measure_with_unit:
            HashMap<u64, as_holder!(ThermodynamicTemperatureMeasureWithUnit)>,
        amount_of_substance_measure_with_unit:
            HashMap<u64, as_holder!(AmountOfSubstanceMeasureWithUnit)>,
        luminous_intensity_measure_with_unit:
            HashMap<u64, as_holder!(LuminousIntensityMeasureWithUnit)>,
        plane_angle_measure_with_unit: HashMap<u64, as_holder!(PlaneAngleMeasureWithUnit)>,
        solid_angle_measure_with_unit: HashMap<u64, as_holder!(SolidAngleMeasureWithUnit)>,
        area_measure_with_unit: HashMap<u64, as_holder!(AreaMeasureWithUnit)>,
        volume_measure_with_unit: HashMap<u64, as_holder!(VolumeMeasureWithUnit)>,
        ratio_measure_with_unit: HashMap<u64, as_holder!(RatioMeasureWithUnit)>,
        geometric_representation_context: HashMap<u64, as_holder!(GeometricRepresentationContext)>,
        geometric_representation_item: HashMap<u64, as_holder!(GeometricRepresentationItem)>,
        point: HashMap<u64, as_holder!(Point)>,
        cartesian_point: HashMap<u64, as_holder!(CartesianPoint)>,
        cylindrical_point: HashMap<u64, as_holder!(CylindricalPoint)>,
        spherical_point: HashMap<u64, as_holder!(SphericalPoint)>,
        polar_point: HashMap<u64, as_holder!(PolarPoint)>,
        point_on_curve: HashMap<u64, as_holder!(PointOnCurve)>,
        point_on_surface: HashMap<u64, as_holder!(PointOnSurface)>,
        point_replica: HashMap<u64, as_holder!(PointReplica)>,
        degenerate_pcurve: HashMap<u64, as_holder!(DegeneratePcurve)>,
        evaluated_degenerate_pcurve: HashMap<u64, as_holder!(EvaluatedDegeneratePcurve)>,
        direction: HashMap<u64, as_holder!(Direction)>,
        vector: HashMap<u64, as_holder!(Vector)>,
        placement: HashMap<u64, as_holder!(Placement)>,
        axis1_placement: HashMap<u64, as_holder!(Axis1Placement)>,
        axis2_placement_2d: HashMap<u64, as_holder!(Axis2Placement2D)>,
        axis2_placement_3d: HashMap<u64, as_holder!(Axis2Placement3D)>,
        cartesian_transformation_operator:
            HashMap<u64, as_holder!(CartesianTransformationOperator)>,
        cartesian_transformation_operator_3d:
            HashMap<u64, as_holder!(CartesianTransformationOperator3D)>,
        cartesian_transformation_operator_2d:
            HashMap<u64, as_holder!(CartesianTransformationOperator2D)>,
        curve: HashMap<u64, as_holder!(Curve)>,
        line: HashMap<u64, as_holder!(Line)>,
        conic: HashMap<u64, as_holder!(Conic)>,
        circle: HashMap<u64, as_holder!(Circle)>,
        ellipse: HashMap<u64, as_holder!(Ellipse)>,
        hyperbola: HashMap<u64, as_holder!(Hyperbola)>,
        parabola: HashMap<u64, as_holder!(Parabola)>,
        bounded_curve: HashMap<u64, as_holder!(BoundedCurve)>,
        polyline: HashMap<u64, as_holder!(Polyline)>,
        b_spline_curve: HashMap<u64, as_holder!(BSplineCurve)>,
        b_spline_curve_with_knots: HashMap<u64, as_holder!(BSplineCurveWithKnots)>,
        uniform_curve: HashMap<u64, as_holder!(UniformCurve)>,
        quasi_uniform_curve: HashMap<u64, as_holder!(QuasiUniformCurve)>,
        bezier_curve: HashMap<u64, as_holder!(BezierCurve)>,
        rational_b_spline_curve: HashMap<u64, as_holder!(RationalBSplineCurve)>,
        trimmed_curve: HashMap<u64, as_holder!(TrimmedCurve)>,
        composite_curve: HashMap<u64, as_holder!(CompositeCurve)>,
        composite_curve_segment: HashMap<u64, as_holder!(CompositeCurveSegment)>,
        reparametrised_composite_curve_segment:
            HashMap<u64, as_holder!(ReparametrisedCompositeCurveSegment)>,
        pcurve: HashMap<u64, as_holder!(Pcurve)>,
        bounded_pcurve: HashMap<u64, as_holder!(BoundedPcurve)>,
        surface_curve: HashMap<u64, as_holder!(SurfaceCurve)>,
        intersection_curve: HashMap<u64, as_holder!(IntersectionCurve)>,
        seam_curve: HashMap<u64, as_holder!(SeamCurve)>,
        bounded_surface_curve: HashMap<u64, as_holder!(BoundedSurfaceCurve)>,
        composite_curve_on_surface: HashMap<u64, as_holder!(CompositeCurveOnSurface)>,
        offset_curve_2d: HashMap<u64, as_holder!(OffsetCurve2D)>,
        offset_curve_3d: HashMap<u64, as_holder!(OffsetCurve3D)>,
        curve_replica: HashMap<u64, as_holder!(CurveReplica)>,
        surface: HashMap<u64, as_holder!(Surface)>,
        elementary_surface: HashMap<u64, as_holder!(ElementarySurface)>,
        plane: HashMap<u64, as_holder!(Plane)>,
        cylindrical_surface: HashMap<u64, as_holder!(CylindricalSurface)>,
        conical_surface: HashMap<u64, as_holder!(ConicalSurface)>,
        spherical_surface: HashMap<u64, as_holder!(SphericalSurface)>,
        toroidal_surface: HashMap<u64, as_holder!(ToroidalSurface)>,
        degenerate_toroidal_surface: HashMap<u64, as_holder!(DegenerateToroidalSurface)>,
        swept_surface: HashMap<u64, as_holder!(SweptSurface)>,
        surface_of_linear_extrusion: HashMap<u64, as_holder!(SurfaceOfLinearExtrusion)>,
        surface_of_revolution: HashMap<u64, as_holder!(SurfaceOfRevolution)>,
        bounded_surface: HashMap<u64, as_holder!(BoundedSurface)>,
        b_spline_surface: HashMap<u64, as_holder!(BSplineSurface)>,
        b_spline_surface_with_knots: HashMap<u64, as_holder!(BSplineSurfaceWithKnots)>,
        uniform_surface: HashMap<u64, as_holder!(UniformSurface)>,
        quasi_uniform_surface: HashMap<u64, as_holder!(QuasiUniformSurface)>,
        bezier_surface: HashMap<u64, as_holder!(BezierSurface)>,
        rational_b_spline_surface: HashMap<u64, as_holder!(RationalBSplineSurface)>,
        rectangular_trimmed_surface: HashMap<u64, as_holder!(RectangularTrimmedSurface)>,
        curve_bounded_surface: HashMap<u64, as_holder!(CurveBoundedSurface)>,
        boundary_curve: HashMap<u64, as_holder!(BoundaryCurve)>,
        outer_boundary_curve: HashMap<u64, as_holder!(OuterBoundaryCurve)>,
        rectangular_composite_surface: HashMap<u64, as_holder!(RectangularCompositeSurface)>,
        surface_patch: HashMap<u64, as_holder!(SurfacePatch)>,
        offset_surface: HashMap<u64, as_holder!(OffsetSurface)>,
        surface_replica: HashMap<u64, as_holder!(SurfaceReplica)>,
        topological_representation_item: HashMap<u64, as_holder!(TopologicalRepresentationItem)>,
        vertex: HashMap<u64, as_holder!(Vertex)>,
        vertex_point: HashMap<u64, as_holder!(VertexPoint)>,
        edge: HashMap<u64, as_holder!(Edge)>,
        edge_curve: HashMap<u64, as_holder!(EdgeCurve)>,
        oriented_edge: HashMap<u64, as_holder!(OrientedEdge)>,
        path: HashMap<u64, as_holder!(Path)>,
        oriented_path: HashMap<u64, as_holder!(OrientedPath)>,
        open_path: HashMap<u64, as_holder!(OpenPath)>,
        r#loop: HashMap<u64, as_holder!(Loop)>,
        vertex_loop: HashMap<u64, as_holder!(VertexLoop)>,
        edge_loop: HashMap<u64, as_holder!(EdgeLoop)>,
        poly_loop: HashMap<u64, as_holder!(PolyLoop)>,
        face_bound: HashMap<u64, as_holder!(FaceBound)>,
        face_outer_bound: HashMap<u64, as_holder!(FaceOuterBound)>,
        face: HashMap<u64, as_holder!(Face)>,
        face_surface: HashMap<u64, as_holder!(FaceSurface)>,
        oriented_face: HashMap<u64, as_holder!(OrientedFace)>,
        subface: HashMap<u64, as_holder!(Subface)>,
        connected_face_set: HashMap<u64, as_holder!(ConnectedFaceSet)>,
        vertex_shell: HashMap<u64, as_holder!(VertexShell)>,
        wire_shell: HashMap<u64, as_holder!(WireShell)>,
        open_shell: HashMap<u64, as_holder!(OpenShell)>,
        oriented_open_shell: HashMap<u64, as_holder!(OrientedOpenShell)>,
        closed_shell: HashMap<u64, as_holder!(ClosedShell)>,
        oriented_closed_shell: HashMap<u64, as_holder!(OrientedClosedShell)>,
        connected_edge_set: HashMap<u64, as_holder!(ConnectedEdgeSet)>,
        solid_model: HashMap<u64, as_holder!(SolidModel)>,
        manifold_solid_brep: HashMap<u64, as_holder!(ManifoldSolidBrep)>,
        brep_with_voids: HashMap<u64, as_holder!(BrepWithVoids)>,
        faceted_brep: HashMap<u64, as_holder!(FacetedBrep)>,
        csg_solid: HashMap<u64, as_holder!(CsgSolid)>,
        boolean_result: HashMap<u64, as_holder!(BooleanResult)>,
        sphere: HashMap<u64, as_holder!(Sphere)>,
        right_circular_cone: HashMap<u64, as_holder!(RightCircularCone)>,
        right_circular_cylinder: HashMap<u64, as_holder!(RightCircularCylinder)>,
        torus: HashMap<u64, as_holder!(Torus)>,
        block: HashMap<u64, as_holder!(Block)>,
        right_angular_wedge: HashMap<u64, as_holder!(RightAngularWedge)>,
        swept_face_solid: HashMap<u64, as_holder!(SweptFaceSolid)>,
        extruded_face_solid: HashMap<u64, as_holder!(ExtrudedFaceSolid)>,
        revolved_face_solid: HashMap<u64, as_holder!(RevolvedFaceSolid)>,
        swept_area_solid: HashMap<u64, as_holder!(SweptAreaSolid)>,
        extruded_area_solid: HashMap<u64, as_holder!(ExtrudedAreaSolid)>,
        revolved_area_solid: HashMap<u64, as_holder!(RevolvedAreaSolid)>,
        half_space_solid: HashMap<u64, as_holder!(HalfSpaceSolid)>,
        boxed_half_space: HashMap<u64, as_holder!(BoxedHalfSpace)>,
        box_domain: HashMap<u64, as_holder!(BoxDomain)>,
        solid_replica: HashMap<u64, as_holder!(SolidReplica)>,
        shell_based_surface_model: HashMap<u64, as_holder!(ShellBasedSurfaceModel)>,
        face_based_surface_model: HashMap<u64, as_holder!(FaceBasedSurfaceModel)>,
        shell_based_wireframe_model: HashMap<u64, as_holder!(ShellBasedWireframeModel)>,
        edge_based_wireframe_model: HashMap<u64, as_holder!(EdgeBasedWireframeModel)>,
        geometric_set: HashMap<u64, as_holder!(GeometricSet)>,
        geometric_curve_set: HashMap<u64, as_holder!(GeometricCurveSet)>,
        geometric_set_replica: HashMap<u64, as_holder!(GeometricSetReplica)>,
        uncertainty_measure_with_unit: HashMap<u64, as_holder!(UncertaintyMeasureWithUnit)>,
        representation_context: HashMap<u64, as_holder!(RepresentationContext)>,
        global_uncertainty_assigned_context:
            HashMap<u64, as_holder!(GlobalUncertaintyAssignedContext)>,
        representation_item: HashMap<u64, as_holder!(RepresentationItem)>,
        representation: HashMap<u64, as_holder!(Representation)>,
        representation_relationship: HashMap<u64, as_holder!(RepresentationRelationship)>,
        item_defined_transformation: HashMap<u64, as_holder!(ItemDefinedTransformation)>,
        functionally_defined_transformation:
            HashMap<u64, as_holder!(FunctionallyDefinedTransformation)>,
        representation_relationship_with_transformation:
            HashMap<u64, as_holder!(RepresentationRelationshipWithTransformation)>,
        representation_map: HashMap<u64, as_holder!(RepresentationMap)>,
        mapped_item: HashMap<u64, as_holder!(MappedItem)>,
        definitional_representation: HashMap<u64, as_holder!(DefinitionalRepresentation)>,
        parametric_representation_context:
            HashMap<u64, as_holder!(ParametricRepresentationContext)>,
        founded_item: HashMap<u64, as_holder!(FoundedItem)>,
        list_of_reversible_topology_item: HashMap<u64, as_holder!(ListOfReversibleTopologyItem)>,
        set_of_reversible_topology_item: HashMap<u64, as_holder!(SetOfReversibleTopologyItem)>,
    }
    impl Tables {
        pub fn application_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApplicationContext>> + 'table {
            self.application_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn application_protocol_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApplicationProtocolDefinition>> + 'table {
            self.application_protocol_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn application_context_element_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApplicationContextElement>> + 'table {
            self.application_context_element
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductContext>> + 'table {
            self.product_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionContext>> + 'table {
            self.product_definition_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_concept_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductConceptContext>> + 'table {
            self.product_concept_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn library_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LibraryContext>> + 'table {
            self.library_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Product>> + 'table {
            self.product
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_category_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductCategory>> + 'table {
            self.product_category
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_related_product_category_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductRelatedProductCategory>> + 'table {
            self.product_related_product_category
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_category_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductCategoryRelationship>> + 'table {
            self.product_category_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_formation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionFormation>> + 'table {
            self.product_definition_formation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_formation_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionFormationRelationship>> + 'table {
            self.product_definition_formation_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_formation_with_specified_source_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionFormationWithSpecifiedSource>> + 'table
        {
            self.product_definition_formation_with_specified_source
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinition>> + 'table {
            self.product_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_with_associated_documents_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionWithAssociatedDocuments>> + 'table
        {
            self.product_definition_with_associated_documents
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionRelationship>> + 'table {
            self.product_definition_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_substitute_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionSubstitute>> + 'table {
            self.product_definition_substitute
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionEffectivity>> + 'table {
            self.product_definition_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn characterized_object_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CharacterizedObject>> + 'table {
            self.characterized_object
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn property_definition_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PropertyDefinition>> + 'table {
            self.property_definition
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn product_definition_shape_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ProductDefinitionShape>> + 'table {
            self.product_definition_shape
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_aspect_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeAspect>> + 'table {
            self.shape_aspect
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_aspect_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeAspectRelationship>> + 'table {
            self.shape_aspect_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeRepresentation>> + 'table {
            self.shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn property_definition_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PropertyDefinitionRepresentation>> + 'table {
            self.property_definition_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_representation_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeRepresentationRelationship>> + 'table {
            self.shape_representation_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn context_dependent_shape_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContextDependentShapeRepresentation>> + 'table {
            self.context_dependent_shape_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shape_definition_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShapeDefinitionRepresentation>> + 'table {
            self.shape_definition_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn name_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<NameAssignment>> + 'table {
            self.name_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn external_referent_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternalReferentAssignment>> + 'table {
            self.external_referent_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn library_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LibraryAssignment>> + 'table {
            self.library_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_reference_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentReference>> + 'table {
            self.document_reference
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_request_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionRequestAssignment>> + 'table {
            self.action_request_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionAssignment>> + 'table {
            self.action_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn certification_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CertificationAssignment>> + 'table {
            self.certification_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalAssignment>> + 'table {
            self.approval_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn contract_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContractAssignment>> + 'table {
            self.contract_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn security_classification_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SecurityClassificationAssignment>> + 'table {
            self.security_classification_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAssignment>> + 'table {
            self.person_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organization_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationAssignment>> + 'table {
            self.organization_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_and_organization_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAndOrganizationAssignment>> + 'table {
            self.person_and_organization_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateAssignment>> + 'table {
            self.date_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn time_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TimeAssignment>> + 'table {
            self.time_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_and_time_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateAndTimeAssignment>> + 'table {
            self.date_and_time_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn group_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GroupAssignment>> + 'table {
            self.group_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn effectivity_assignment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EffectivityAssignment>> + 'table {
            self.effectivity_assignment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_type_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentType>> + 'table {
            self.document_type
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Document>> + 'table {
            self.document
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_with_class_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentWithClass>> + 'table {
            self.document_with_class
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_usage_constraint_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentUsageConstraint>> + 'table {
            self.document_usage_constraint
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn document_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DocumentRelationship>> + 'table {
            self.document_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_iter<'table>(&'table self) -> impl Iterator<Item = Result<Action>> + 'table {
            self.action
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn executed_action_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExecutedAction>> + 'table {
            self.executed_action
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn directed_action_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DirectedAction>> + 'table {
            self.directed_action
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_status_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionStatus>> + 'table {
            self.action_status
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_request_status_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionRequestStatus>> + 'table {
            self.action_request_status
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionRelationship>> + 'table {
            self.action_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_method_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionMethod>> + 'table {
            self.action_method
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_request_solution_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionRequestSolution>> + 'table {
            self.action_request_solution
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_method_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionMethodRelationship>> + 'table {
            self.action_method_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn versioned_action_request_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VersionedActionRequest>> + 'table {
            self.versioned_action_request
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_directive_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionDirective>> + 'table {
            self.action_directive
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_resource_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionResource>> + 'table {
            self.action_resource
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_resource_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionResourceRelationship>> + 'table {
            self.action_resource_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn action_resource_type_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ActionResourceType>> + 'table {
            self.action_resource_type
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn certification_type_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CertificationType>> + 'table {
            self.certification_type
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn certification_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Certification>> + 'table {
            self.certification
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_status_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalStatus>> + 'table {
            self.approval_status
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Approval>> + 'table {
            self.approval
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_date_time_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalDateTime>> + 'table {
            self.approval_date_time
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_person_organization_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalPersonOrganization>> + 'table {
            self.approval_person_organization
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalRole>> + 'table {
            self.approval_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn approval_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ApprovalRelationship>> + 'table {
            self.approval_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn security_classification_level_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SecurityClassificationLevel>> + 'table {
            self.security_classification_level
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn security_classification_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SecurityClassification>> + 'table {
            self.security_classification
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn contract_type_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContractType>> + 'table {
            self.contract_type
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn contract_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Contract>> + 'table {
            self.contract
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn address_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Address>> + 'table {
            self.address
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn personal_address_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonalAddress>> + 'table {
            self.personal_address
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organizational_address_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationalAddress>> + 'table {
            self.organizational_address
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_iter<'table>(&'table self) -> impl Iterator<Item = Result<Person>> + 'table {
            self.person
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organization_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Organization>> + 'table {
            self.organization
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organizational_project_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationalProject>> + 'table {
            self.organizational_project
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_and_organization_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAndOrganization>> + 'table {
            self.person_and_organization
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organization_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationRelationship>> + 'table {
            self.organization_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_and_organization_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonAndOrganizationRole>> + 'table {
            self.person_and_organization_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn person_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PersonRole>> + 'table {
            self.person_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn organization_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrganizationRole>> + 'table {
            self.organization_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_iter<'table>(&'table self) -> impl Iterator<Item = Result<Date>> + 'table {
            self.date
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn calendar_date_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CalendarDate>> + 'table {
            self.calendar_date
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ordinal_date_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrdinalDate>> + 'table {
            self.ordinal_date
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn week_of_year_and_day_date_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<WeekOfYearAndDayDate>> + 'table {
            self.week_of_year_and_day_date
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn coordinated_universal_time_offset_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CoordinatedUniversalTimeOffset>> + 'table {
            self.coordinated_universal_time_offset
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn local_time_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LocalTime>> + 'table {
            self.local_time
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_and_time_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateAndTime>> + 'table {
            self.date_and_time
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_time_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateTimeRole>> + 'table {
            self.date_time_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn date_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DateRole>> + 'table {
            self.date_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn time_role_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TimeRole>> + 'table {
            self.time_role
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn group_iter<'table>(&'table self) -> impl Iterator<Item = Result<Group>> + 'table {
            self.group
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn group_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GroupRelationship>> + 'table {
            self.group_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Effectivity>> + 'table {
            self.effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn serial_numbered_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SerialNumberedEffectivity>> + 'table {
            self.serial_numbered_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dated_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DatedEffectivity>> + 'table {
            self.dated_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn lot_effectivity_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LotEffectivity>> + 'table {
            self.lot_effectivity
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn external_source_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternalSource>> + 'table {
            self.external_source
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn external_source_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternalSourceRelationship>> + 'table {
            self.external_source_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pre_defined_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PreDefinedItem>> + 'table {
            self.pre_defined_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn externally_defined_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExternallyDefinedItem>> + 'table {
            self.externally_defined_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn named_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<NamedUnit>> + 'table {
            self.named_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn si_unit_iter<'table>(&'table self) -> impl Iterator<Item = Result<SiUnit>> + 'table {
            self.si_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn conversion_based_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConversionBasedUnit>> + 'table {
            self.conversion_based_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn context_dependent_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ContextDependentUnit>> + 'table {
            self.context_dependent_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn length_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LengthUnit>> + 'table {
            self.length_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn mass_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MassUnit>> + 'table {
            self.mass_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn time_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TimeUnit>> + 'table {
            self.time_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn electric_current_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ElectricCurrentUnit>> + 'table {
            self.electric_current_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn thermodynamic_temperature_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ThermodynamicTemperatureUnit>> + 'table {
            self.thermodynamic_temperature_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn amount_of_substance_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AmountOfSubstanceUnit>> + 'table {
            self.amount_of_substance_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn luminous_intensity_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LuminousIntensityUnit>> + 'table {
            self.luminous_intensity_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn plane_angle_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PlaneAngleUnit>> + 'table {
            self.plane_angle_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn solid_angle_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SolidAngleUnit>> + 'table {
            self.solid_angle_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn area_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AreaUnit>> + 'table {
            self.area_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn volume_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VolumeUnit>> + 'table {
            self.volume_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ratio_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RatioUnit>> + 'table {
            self.ratio_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn dimensional_exponents_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DimensionalExponents>> + 'table {
            self.dimensional_exponents
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn derived_unit_element_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DerivedUnitElement>> + 'table {
            self.derived_unit_element
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn derived_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DerivedUnit>> + 'table {
            self.derived_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn global_unit_assigned_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GlobalUnitAssignedContext>> + 'table {
            self.global_unit_assigned_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MeasureWithUnit>> + 'table {
            self.measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn length_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LengthMeasureWithUnit>> + 'table {
            self.length_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn mass_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MassMeasureWithUnit>> + 'table {
            self.mass_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn time_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TimeMeasureWithUnit>> + 'table {
            self.time_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn electric_current_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ElectricCurrentMeasureWithUnit>> + 'table {
            self.electric_current_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn thermodynamic_temperature_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ThermodynamicTemperatureMeasureWithUnit>> + 'table
        {
            self.thermodynamic_temperature_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn amount_of_substance_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AmountOfSubstanceMeasureWithUnit>> + 'table {
            self.amount_of_substance_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn luminous_intensity_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<LuminousIntensityMeasureWithUnit>> + 'table {
            self.luminous_intensity_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn plane_angle_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PlaneAngleMeasureWithUnit>> + 'table {
            self.plane_angle_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn solid_angle_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SolidAngleMeasureWithUnit>> + 'table {
            self.solid_angle_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn area_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<AreaMeasureWithUnit>> + 'table {
            self.area_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn volume_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VolumeMeasureWithUnit>> + 'table {
            self.volume_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ratio_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RatioMeasureWithUnit>> + 'table {
            self.ratio_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_representation_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricRepresentationContext>> + 'table {
            self.geometric_representation_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_representation_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricRepresentationItem>> + 'table {
            self.geometric_representation_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn point_iter<'table>(&'table self) -> impl Iterator<Item = Result<Point>> + 'table {
            self.point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cartesian_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianPoint>> + 'table {
            self.cartesian_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cylindrical_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CylindricalPoint>> + 'table {
            self.cylindrical_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn spherical_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SphericalPoint>> + 'table {
            self.spherical_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn polar_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PolarPoint>> + 'table {
            self.polar_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn point_on_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PointOnCurve>> + 'table {
            self.point_on_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn point_on_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PointOnSurface>> + 'table {
            self.point_on_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn point_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PointReplica>> + 'table {
            self.point_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn degenerate_pcurve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DegeneratePcurve>> + 'table {
            self.degenerate_pcurve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn evaluated_degenerate_pcurve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EvaluatedDegeneratePcurve>> + 'table {
            self.evaluated_degenerate_pcurve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn direction_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Direction>> + 'table {
            self.direction
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vector_iter<'table>(&'table self) -> impl Iterator<Item = Result<Vector>> + 'table {
            self.vector
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn placement_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Placement>> + 'table {
            self.placement
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn axis1_placement_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis1Placement>> + 'table {
            self.axis1_placement
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn axis2_placement_2d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis2Placement2D>> + 'table {
            self.axis2_placement_2d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn axis2_placement_3d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Axis2Placement3D>> + 'table {
            self.axis2_placement_3d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cartesian_transformation_operator_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianTransformationOperator>> + 'table {
            self.cartesian_transformation_operator
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cartesian_transformation_operator_3d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianTransformationOperator3D>> + 'table {
            self.cartesian_transformation_operator_3d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cartesian_transformation_operator_2d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CartesianTransformationOperator2D>> + 'table {
            self.cartesian_transformation_operator_2d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_iter<'table>(&'table self) -> impl Iterator<Item = Result<Curve>> + 'table {
            self.curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn line_iter<'table>(&'table self) -> impl Iterator<Item = Result<Line>> + 'table {
            self.line
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn conic_iter<'table>(&'table self) -> impl Iterator<Item = Result<Conic>> + 'table {
            self.conic
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn circle_iter<'table>(&'table self) -> impl Iterator<Item = Result<Circle>> + 'table {
            self.circle
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn ellipse_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Ellipse>> + 'table {
            self.ellipse
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn hyperbola_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Hyperbola>> + 'table {
            self.hyperbola
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn parabola_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Parabola>> + 'table {
            self.parabola
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bounded_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedCurve>> + 'table {
            self.bounded_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn polyline_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Polyline>> + 'table {
            self.polyline
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn b_spline_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineCurve>> + 'table {
            self.b_spline_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn b_spline_curve_with_knots_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineCurveWithKnots>> + 'table {
            self.b_spline_curve_with_knots
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn uniform_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<UniformCurve>> + 'table {
            self.uniform_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn quasi_uniform_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<QuasiUniformCurve>> + 'table {
            self.quasi_uniform_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bezier_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BezierCurve>> + 'table {
            self.bezier_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn rational_b_spline_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RationalBSplineCurve>> + 'table {
            self.rational_b_spline_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn trimmed_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TrimmedCurve>> + 'table {
            self.trimmed_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeCurve>> + 'table {
            self.composite_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_curve_segment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeCurveSegment>> + 'table {
            self.composite_curve_segment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn reparametrised_composite_curve_segment_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ReparametrisedCompositeCurveSegment>> + 'table {
            self.reparametrised_composite_curve_segment
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn pcurve_iter<'table>(&'table self) -> impl Iterator<Item = Result<Pcurve>> + 'table {
            self.pcurve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bounded_pcurve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedPcurve>> + 'table {
            self.bounded_pcurve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceCurve>> + 'table {
            self.surface_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn intersection_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<IntersectionCurve>> + 'table {
            self.intersection_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn seam_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SeamCurve>> + 'table {
            self.seam_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bounded_surface_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedSurfaceCurve>> + 'table {
            self.bounded_surface_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn composite_curve_on_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CompositeCurveOnSurface>> + 'table {
            self.composite_curve_on_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn offset_curve_2d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OffsetCurve2D>> + 'table {
            self.offset_curve_2d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn offset_curve_3d_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OffsetCurve3D>> + 'table {
            self.offset_curve_3d
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveReplica>> + 'table {
            self.curve_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Surface>> + 'table {
            self.surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn elementary_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ElementarySurface>> + 'table {
            self.elementary_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn plane_iter<'table>(&'table self) -> impl Iterator<Item = Result<Plane>> + 'table {
            self.plane
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn cylindrical_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CylindricalSurface>> + 'table {
            self.cylindrical_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn conical_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConicalSurface>> + 'table {
            self.conical_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn spherical_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SphericalSurface>> + 'table {
            self.spherical_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn toroidal_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ToroidalSurface>> + 'table {
            self.toroidal_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn degenerate_toroidal_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DegenerateToroidalSurface>> + 'table {
            self.degenerate_toroidal_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn swept_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SweptSurface>> + 'table {
            self.swept_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_of_linear_extrusion_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceOfLinearExtrusion>> + 'table {
            self.surface_of_linear_extrusion
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_of_revolution_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceOfRevolution>> + 'table {
            self.surface_of_revolution
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bounded_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundedSurface>> + 'table {
            self.bounded_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn b_spline_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineSurface>> + 'table {
            self.b_spline_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn b_spline_surface_with_knots_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BSplineSurfaceWithKnots>> + 'table {
            self.b_spline_surface_with_knots
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn uniform_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<UniformSurface>> + 'table {
            self.uniform_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn quasi_uniform_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<QuasiUniformSurface>> + 'table {
            self.quasi_uniform_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn bezier_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BezierSurface>> + 'table {
            self.bezier_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn rational_b_spline_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RationalBSplineSurface>> + 'table {
            self.rational_b_spline_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn rectangular_trimmed_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RectangularTrimmedSurface>> + 'table {
            self.rectangular_trimmed_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn curve_bounded_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CurveBoundedSurface>> + 'table {
            self.curve_bounded_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn boundary_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoundaryCurve>> + 'table {
            self.boundary_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn outer_boundary_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OuterBoundaryCurve>> + 'table {
            self.outer_boundary_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn rectangular_composite_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RectangularCompositeSurface>> + 'table {
            self.rectangular_composite_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_patch_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfacePatch>> + 'table {
            self.surface_patch
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn offset_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OffsetSurface>> + 'table {
            self.offset_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn surface_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SurfaceReplica>> + 'table {
            self.surface_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn topological_representation_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<TopologicalRepresentationItem>> + 'table {
            self.topological_representation_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_iter<'table>(&'table self) -> impl Iterator<Item = Result<Vertex>> + 'table {
            self.vertex
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_point_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VertexPoint>> + 'table {
            self.vertex_point
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_iter<'table>(&'table self) -> impl Iterator<Item = Result<Edge>> + 'table {
            self.edge
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_curve_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EdgeCurve>> + 'table {
            self.edge_curve
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_edge_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedEdge>> + 'table {
            self.oriented_edge
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn path_iter<'table>(&'table self) -> impl Iterator<Item = Result<Path>> + 'table {
            self.path
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_path_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedPath>> + 'table {
            self.oriented_path
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn open_path_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OpenPath>> + 'table {
            self.open_path
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn loop_iter<'table>(&'table self) -> impl Iterator<Item = Result<Loop>> + 'table {
            self.r#loop
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_loop_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VertexLoop>> + 'table {
            self.vertex_loop
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_loop_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EdgeLoop>> + 'table {
            self.edge_loop
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn poly_loop_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<PolyLoop>> + 'table {
            self.poly_loop
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_bound_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FaceBound>> + 'table {
            self.face_bound
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_outer_bound_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FaceOuterBound>> + 'table {
            self.face_outer_bound
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_iter<'table>(&'table self) -> impl Iterator<Item = Result<Face>> + 'table {
            self.face
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_surface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FaceSurface>> + 'table {
            self.face_surface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_face_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedFace>> + 'table {
            self.oriented_face
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn subface_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Subface>> + 'table {
            self.subface
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn connected_face_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConnectedFaceSet>> + 'table {
            self.connected_face_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn vertex_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<VertexShell>> + 'table {
            self.vertex_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn wire_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<WireShell>> + 'table {
            self.wire_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn open_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OpenShell>> + 'table {
            self.open_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_open_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedOpenShell>> + 'table {
            self.oriented_open_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn closed_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ClosedShell>> + 'table {
            self.closed_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn oriented_closed_shell_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<OrientedClosedShell>> + 'table {
            self.oriented_closed_shell
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn connected_edge_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ConnectedEdgeSet>> + 'table {
            self.connected_edge_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn solid_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SolidModel>> + 'table {
            self.solid_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn manifold_solid_brep_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ManifoldSolidBrep>> + 'table {
            self.manifold_solid_brep
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn brep_with_voids_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BrepWithVoids>> + 'table {
            self.brep_with_voids
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn faceted_brep_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FacetedBrep>> + 'table {
            self.faceted_brep
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn csg_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<CsgSolid>> + 'table {
            self.csg_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn boolean_result_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BooleanResult>> + 'table {
            self.boolean_result
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn sphere_iter<'table>(&'table self) -> impl Iterator<Item = Result<Sphere>> + 'table {
            self.sphere
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn right_circular_cone_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RightCircularCone>> + 'table {
            self.right_circular_cone
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn right_circular_cylinder_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RightCircularCylinder>> + 'table {
            self.right_circular_cylinder
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn torus_iter<'table>(&'table self) -> impl Iterator<Item = Result<Torus>> + 'table {
            self.torus
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn block_iter<'table>(&'table self) -> impl Iterator<Item = Result<Block>> + 'table {
            self.block
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn right_angular_wedge_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RightAngularWedge>> + 'table {
            self.right_angular_wedge
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn swept_face_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SweptFaceSolid>> + 'table {
            self.swept_face_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn extruded_face_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExtrudedFaceSolid>> + 'table {
            self.extruded_face_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn revolved_face_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RevolvedFaceSolid>> + 'table {
            self.revolved_face_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn swept_area_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SweptAreaSolid>> + 'table {
            self.swept_area_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn extruded_area_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ExtrudedAreaSolid>> + 'table {
            self.extruded_area_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn revolved_area_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RevolvedAreaSolid>> + 'table {
            self.revolved_area_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn half_space_solid_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<HalfSpaceSolid>> + 'table {
            self.half_space_solid
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn boxed_half_space_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoxedHalfSpace>> + 'table {
            self.boxed_half_space
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn box_domain_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<BoxDomain>> + 'table {
            self.box_domain
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn solid_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SolidReplica>> + 'table {
            self.solid_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shell_based_surface_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShellBasedSurfaceModel>> + 'table {
            self.shell_based_surface_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn face_based_surface_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FaceBasedSurfaceModel>> + 'table {
            self.face_based_surface_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn shell_based_wireframe_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ShellBasedWireframeModel>> + 'table {
            self.shell_based_wireframe_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn edge_based_wireframe_model_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<EdgeBasedWireframeModel>> + 'table {
            self.edge_based_wireframe_model
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricSet>> + 'table {
            self.geometric_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_curve_set_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricCurveSet>> + 'table {
            self.geometric_curve_set
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn geometric_set_replica_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GeometricSetReplica>> + 'table {
            self.geometric_set_replica
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn uncertainty_measure_with_unit_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<UncertaintyMeasureWithUnit>> + 'table {
            self.uncertainty_measure_with_unit
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationContext>> + 'table {
            self.representation_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn global_uncertainty_assigned_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<GlobalUncertaintyAssignedContext>> + 'table {
            self.global_uncertainty_assigned_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationItem>> + 'table {
            self.representation_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<Representation>> + 'table {
            self.representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_relationship_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationRelationship>> + 'table {
            self.representation_relationship
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn item_defined_transformation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ItemDefinedTransformation>> + 'table {
            self.item_defined_transformation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn functionally_defined_transformation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FunctionallyDefinedTransformation>> + 'table {
            self.functionally_defined_transformation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_relationship_with_transformation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationRelationshipWithTransformation>> + 'table
        {
            self.representation_relationship_with_transformation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn representation_map_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<RepresentationMap>> + 'table {
            self.representation_map
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn mapped_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<MappedItem>> + 'table {
            self.mapped_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn definitional_representation_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<DefinitionalRepresentation>> + 'table {
            self.definitional_representation
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn parametric_representation_context_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ParametricRepresentationContext>> + 'table {
            self.parametric_representation_context
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn founded_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<FoundedItem>> + 'table {
            self.founded_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn list_of_reversible_topology_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<ListOfReversibleTopologyItem>> + 'table {
            self.list_of_reversible_topology_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
        pub fn set_of_reversible_topology_item_iter<'table>(
            &'table self,
        ) -> impl Iterator<Item = Result<SetOfReversibleTopologyItem>> + 'table {
            self.set_of_reversible_topology_item
                .values()
                .cloned()
                .map(move |value| value.into_owned(&self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum Source {
        Made,
        Bought,
        NotKnown,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedDefinition {
        # [holder (field = characterized_object)]
        #[holder(use_place_holder)]
        CharacterizedObject(Box<CharacterizedObject>),
        #[holder(use_place_holder)]
        CharacterizedProductDefinition(Box<CharacterizedProductDefinition>),
        #[holder(use_place_holder)]
        ShapeDefinition(Box<ShapeDefinition>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CharacterizedProductDefinition {
        #[holder(use_place_holder)]
        ProductDefinition(ProductDefinitionAny),
        # [holder (field = product_definition_relationship)]
        #[holder(use_place_holder)]
        ProductDefinitionRelationship(Box<ProductDefinitionRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ShapeDefinition {
        # [holder (field = product_definition_shape)]
        #[holder(use_place_holder)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
        # [holder (field = shape_aspect)]
        #[holder(use_place_holder)]
        ShapeAspect(Box<ShapeAspect>),
        # [holder (field = shape_aspect_relationship)]
        #[holder(use_place_holder)]
        ShapeAspectRelationship(Box<ShapeAspectRelationship>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SupportedItem {
        # [holder (field = action_directive)]
        #[holder(use_place_holder)]
        ActionDirective(Box<ActionDirective>),
        #[holder(use_place_holder)]
        Action(ActionAny),
        # [holder (field = action_method)]
        #[holder(use_place_holder)]
        ActionMethod(Box<ActionMethod>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PersonOrganizationSelect {
        # [holder (field = person)]
        #[holder(use_place_holder)]
        Person(Box<Person>),
        # [holder (field = organization)]
        #[holder(use_place_holder)]
        Organization(Box<Organization>),
        # [holder (field = person_and_organization)]
        #[holder(use_place_holder)]
        PersonAndOrganization(Box<PersonAndOrganization>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateTimeSelect {
        #[holder(use_place_holder)]
        Date(DateAny),
        # [holder (field = local_time)]
        #[holder(use_place_holder)]
        LocalTime(Box<LocalTime>),
        # [holder (field = date_and_time)]
        #[holder(use_place_holder)]
        DateAndTime(Box<DateAndTime>),
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct YearNumber(pub i64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct MonthInYearNumber(pub i64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct WeekInYearNumber(pub i64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct DayInWeekNumber(pub i64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct DayInMonthNumber(pub i64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct DayInYearNumber(pub i64);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum AheadOrBehind {
        Ahead,
        Behind,
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct HourInDay(pub i64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct MinuteInHour(pub i64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct SecondInMinute(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct Message(pub String);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SourceItem {
        Identifier(Identifier),
        Message(Message),
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct Identifier(pub String);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct Label(pub String);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct Text(pub String);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureValue {
        LengthMeasure(LengthMeasure),
        MassMeasure(MassMeasure),
        TimeMeasure(TimeMeasure),
        ElectricCurrentMeasure(ElectricCurrentMeasure),
        ThermodynamicTemperatureMeasure(ThermodynamicTemperatureMeasure),
        AmountOfSubstanceMeasure(AmountOfSubstanceMeasure),
        LuminousIntensityMeasure(LuminousIntensityMeasure),
        PlaneAngleMeasure(PlaneAngleMeasure),
        SolidAngleMeasure(SolidAngleMeasure),
        AreaMeasure(AreaMeasure),
        VolumeMeasure(VolumeMeasure),
        RatioMeasure(RatioMeasure),
        ParameterValue(ParameterValue),
        NumericMeasure(NumericMeasure),
        ContextDependentMeasure(ContextDependentMeasure),
        DescriptiveMeasure(DescriptiveMeasure),
        PositiveLengthMeasure(PositiveLengthMeasure),
        PositivePlaneAngleMeasure(PositivePlaneAngleMeasure),
        PositiveRatioMeasure(PositiveRatioMeasure),
        CountMeasure(CountMeasure),
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct LengthMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct MassMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct TimeMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct ElectricCurrentMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct ThermodynamicTemperatureMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct AmountOfSubstanceMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct LuminousIntensityMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct PlaneAngleMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct SolidAngleMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct AreaMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct VolumeMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct RatioMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct ParameterValue(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct NumericMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct PositiveLengthMeasure(pub LengthMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct PositivePlaneAngleMeasure(pub PlaneAngleMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct PositiveRatioMeasure(pub RatioMeasure);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct ContextDependentMeasure(pub f64);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct DescriptiveMeasure(pub String);
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct CountMeasure(pub f64);
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Unit {
        #[holder(use_place_holder)]
        NamedUnit(NamedUnitAny),
        # [holder (field = derived_unit)]
        #[holder(use_place_holder)]
        DerivedUnit(Box<DerivedUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum SiUnitName {
        Metre,
        Gram,
        Second,
        Ampere,
        Kelvin,
        Mole,
        Candela,
        Radian,
        Steradian,
        Hertz,
        Newton,
        Pascal,
        Joule,
        Watt,
        Coulomb,
        Volt,
        Farad,
        Ohm,
        Siemens,
        Weber,
        Tesla,
        Henry,
        DegreeCelsius,
        Lumen,
        Lux,
        Becquerel,
        Gray,
        Sievert,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum SiPrefix {
        Exa,
        Peta,
        Tera,
        Giga,
        Mega,
        Kilo,
        Hecto,
        Deca,
        Deci,
        Centi,
        Milli,
        Micro,
        Nano,
        Pico,
        Femto,
        Atto,
    }
    #[derive(
        Clone,
        Debug,
        PartialEq,
        AsRef,
        Deref,
        DerefMut,
        From,
        Into,
        :: serde :: Serialize,
        :: serde :: Deserialize,
    )]
    pub struct DimensionCount(pub i64);
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TransitionCode {
        Discontinuous,
        Continuous,
        ContSameGradient,
        ContSameGradientSameCurvature,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum PreferredSurfaceCurveRepresentation {
        Curve3D,
        PcurveS1,
        PcurveS2,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BSplineCurveForm {
        PolylineForm,
        CircularArc,
        EllipticArc,
        ParabolicArc,
        HyperbolicArc,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BSplineSurfaceForm {
        PlaneSurf,
        CylindricalSurf,
        ConicalSurf,
        SphericalSurf,
        ToroidalSurf,
        SurfOfRevolution,
        RuledSurf,
        GeneralisedCone,
        QuadricSurf,
        SurfOfLinearExtrusion,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum KnotType {
        UniformKnots,
        Unspecified,
        QuasiUniformKnots,
        PiecewiseBezierKnots,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum ExtentEnumeration {
        Invalid,
        Zero,
        FiniteNonZero,
        Infinite,
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum TrimmingPreference {
        Cartesian,
        Parameter,
        Unspecified,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Axis2Placement {
        # [holder (field = axis2_placement_2d)]
        #[holder(use_place_holder)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        # [holder (field = axis2_placement_3d)]
        #[holder(use_place_holder)]
        Axis2Placement3D(Box<Axis2Placement3D>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveOnSurface {
        #[holder(use_place_holder)]
        Pcurve(PcurveAny),
        #[holder(use_place_holder)]
        SurfaceCurve(SurfaceCurveAny),
        #[holder(use_place_holder)]
        CompositeCurveOnSurface(CompositeCurveOnSurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PcurveOrSurface {
        #[holder(use_place_holder)]
        Pcurve(PcurveAny),
        #[holder(use_place_holder)]
        Surface(SurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TrimmingSelect {
        #[holder(use_place_holder)]
        CartesianPoint(CartesianPointAny),
        ParameterValue(ParameterValue),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum VectorOrDirection {
        # [holder (field = vector)]
        #[holder(use_place_holder)]
        Vector(Box<Vector>),
        # [holder (field = direction)]
        #[holder(use_place_holder)]
        Direction(Box<Direction>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Shell {
        # [holder (field = vertex_shell)]
        #[holder(use_place_holder)]
        VertexShell(Box<VertexShell>),
        # [holder (field = wire_shell)]
        #[holder(use_place_holder)]
        WireShell(Box<WireShell>),
        #[holder(use_place_holder)]
        OpenShell(OpenShellAny),
        #[holder(use_place_holder)]
        ClosedShell(ClosedShellAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ReversibleTopologyItem {
        #[holder(use_place_holder)]
        Edge(EdgeAny),
        #[holder(use_place_holder)]
        Path(PathAny),
        #[holder(use_place_holder)]
        Face(FaceAny),
        #[holder(use_place_holder)]
        FaceBound(FaceBoundAny),
        #[holder(use_place_holder)]
        ClosedShell(ClosedShellAny),
        #[holder(use_place_holder)]
        OpenShell(OpenShellAny),
    }
    #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = list_of_reversible_topology_item)]
    #[holder(generate_deserialize)]
    pub struct ListOfReversibleTopologyItem(
        #[holder(use_place_holder)] pub Vec<ReversibleTopologyItem>,
    );
    #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, :: ruststep_derive :: Holder)]
    # [holder (table = Tables)]
    # [holder (field = set_of_reversible_topology_item)]
    #[holder(generate_deserialize)]
    pub struct SetOfReversibleTopologyItem(
        #[holder(use_place_holder)] pub Vec<ReversibleTopologyItem>,
    );
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ReversibleTopology {
        #[holder(use_place_holder)]
        ReversibleTopologyItem(Box<ReversibleTopologyItem>),
        #[holder(use_place_holder)]
        ListOfReversibleTopologyItem(Box<ListOfReversibleTopologyItem>),
        #[holder(use_place_holder)]
        SetOfReversibleTopologyItem(Box<SetOfReversibleTopologyItem>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BooleanOperand {
        #[holder(use_place_holder)]
        SolidModel(SolidModelAny),
        #[holder(use_place_holder)]
        HalfSpaceSolid(HalfSpaceSolidAny),
        #[holder(use_place_holder)]
        CsgPrimitive(Box<CsgPrimitive>),
        # [holder (field = boolean_result)]
        #[holder(use_place_holder)]
        BooleanResult(Box<BooleanResult>),
    }
    #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
    pub enum BooleanOperator {
        Union,
        Intersection,
        Difference,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CsgPrimitive {
        # [holder (field = sphere)]
        #[holder(use_place_holder)]
        Sphere(Box<Sphere>),
        # [holder (field = block)]
        #[holder(use_place_holder)]
        Block(Box<Block>),
        # [holder (field = right_angular_wedge)]
        #[holder(use_place_holder)]
        RightAngularWedge(Box<RightAngularWedge>),
        # [holder (field = torus)]
        #[holder(use_place_holder)]
        Torus(Box<Torus>),
        # [holder (field = right_circular_cone)]
        #[holder(use_place_holder)]
        RightCircularCone(Box<RightCircularCone>),
        # [holder (field = right_circular_cylinder)]
        #[holder(use_place_holder)]
        RightCircularCylinder(Box<RightCircularCylinder>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CsgSelect {
        # [holder (field = boolean_result)]
        #[holder(use_place_holder)]
        BooleanResult(Box<BooleanResult>),
        #[holder(use_place_holder)]
        CsgPrimitive(Box<CsgPrimitive>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetSelect {
        #[holder(use_place_holder)]
        Point(PointAny),
        #[holder(use_place_holder)]
        Curve(CurveAny),
        #[holder(use_place_holder)]
        Surface(SurfaceAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceModel {
        # [holder (field = shell_based_surface_model)]
        #[holder(use_place_holder)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
        # [holder (field = face_based_surface_model)]
        #[holder(use_place_holder)]
        FaceBasedSurfaceModel(Box<FaceBasedSurfaceModel>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum WireframeModel {
        # [holder (field = shell_based_wireframe_model)]
        #[holder(use_place_holder)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        # [holder (field = edge_based_wireframe_model)]
        #[holder(use_place_holder)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemSelect {
        #[holder(use_place_holder)]
        FoundedItem(FoundedItemAny),
        #[holder(use_place_holder)]
        RepresentationItem(RepresentationItemAny),
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum Transformation {
        # [holder (field = item_defined_transformation)]
        #[holder(use_place_holder)]
        ItemDefinedTransformation(Box<ItemDefinedTransformation>),
        #[holder(use_place_holder)]
        FunctionallyDefinedTransformation(FunctionallyDefinedTransformationAny),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContext {
        pub application: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_protocol_definition)]
    #[holder(generate_deserialize)]
    pub struct ApplicationProtocolDefinition {
        pub status: Label,
        pub application_interpreted_model_schema_name: Label,
        pub application_protocol_year: YearNumber,
        #[holder(use_place_holder)]
        pub application: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = application_context_element)]
    #[holder(generate_deserialize)]
    pub struct ApplicationContextElement {
        pub name: Label,
        #[holder(use_place_holder)]
        pub frame_of_reference: ApplicationContext,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ApplicationContextElementAny {
        #[holder(use_place_holder)]
        # [holder (field = product_context)]
        ProductContext(Box<ProductContext>),
        #[holder(use_place_holder)]
        # [holder (field = product_definition_context)]
        ProductDefinitionContext(Box<ProductDefinitionContext>),
        #[holder(use_place_holder)]
        # [holder (field = product_concept_context)]
        ProductConceptContext(Box<ProductConceptContext>),
        #[holder(use_place_holder)]
        # [holder (field = library_context)]
        LibraryContext(Box<LibraryContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_context)]
    #[holder(generate_deserialize)]
    pub struct ProductContext {
        #[holder(use_place_holder)]
        pub application_context_element: ApplicationContextElement,
        pub discipline_type: Label,
    }
    impl Into<ApplicationContextElementAny> for ProductContext {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ProductContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_context)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionContext {
        #[holder(use_place_holder)]
        pub application_context_element: ApplicationContextElement,
        pub life_cycle_stage: Label,
    }
    impl Into<ApplicationContextElementAny> for ProductDefinitionContext {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ProductDefinitionContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_concept_context)]
    #[holder(generate_deserialize)]
    pub struct ProductConceptContext {
        #[holder(use_place_holder)]
        pub application_context_element: ApplicationContextElement,
        pub market_segment_type: Label,
    }
    impl Into<ApplicationContextElementAny> for ProductConceptContext {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::ProductConceptContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = library_context)]
    #[holder(generate_deserialize)]
    pub struct LibraryContext {
        #[holder(use_place_holder)]
        pub application_context_element: ApplicationContextElement,
        pub library_reference: Label,
    }
    impl Into<ApplicationContextElementAny> for LibraryContext {
        fn into(self) -> ApplicationContextElementAny {
            ApplicationContextElementAny::LibraryContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product)]
    #[holder(generate_deserialize)]
    pub struct Product {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub frame_of_reference: Vec<ProductContext>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductCategory {
        pub name: Label,
        pub description: Option<Text>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductCategoryAny {
        #[holder(use_place_holder)]
        # [holder (field = product_related_product_category)]
        ProductRelatedProductCategory(Box<ProductRelatedProductCategory>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_related_product_category)]
    #[holder(generate_deserialize)]
    pub struct ProductRelatedProductCategory {
        #[holder(use_place_holder)]
        pub product_category: ProductCategory,
        #[holder(use_place_holder)]
        pub products: Vec<Product>,
    }
    impl Into<ProductCategoryAny> for ProductRelatedProductCategory {
        fn into(self) -> ProductCategoryAny {
            ProductCategoryAny::ProductRelatedProductCategory(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_category_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductCategoryRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub category: ProductCategoryAny,
        #[holder(use_place_holder)]
        pub sub_category: ProductCategoryAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormation {
        pub id: Identifier,
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_product: Product,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionFormationAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_formation_with_specified_source)]
        ProductDefinitionFormationWithSpecifiedSource(
            Box<ProductDefinitionFormationWithSpecifiedSource>,
        ),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormationRelationship {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition_formation: ProductDefinitionFormationAny,
        #[holder(use_place_holder)]
        pub related_product_definition_formation: ProductDefinitionFormationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_formation_with_specified_source)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionFormationWithSpecifiedSource {
        #[holder(use_place_holder)]
        pub product_definition_formation: ProductDefinitionFormation,
        pub make_or_buy: Source,
    }
    impl Into<ProductDefinitionFormationAny> for ProductDefinitionFormationWithSpecifiedSource {
        fn into(self) -> ProductDefinitionFormationAny {
            ProductDefinitionFormationAny::ProductDefinitionFormationWithSpecifiedSource(Box::new(
                self,
            ))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinition {
        pub id: Identifier,
        pub description: Text,
        #[holder(use_place_holder)]
        pub formation: ProductDefinitionFormationAny,
        #[holder(use_place_holder)]
        pub frame_of_reference: ProductDefinitionContext,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ProductDefinitionAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_with_associated_documents)]
        ProductDefinitionWithAssociatedDocuments(Box<ProductDefinitionWithAssociatedDocuments>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_with_associated_documents)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionWithAssociatedDocuments {
        #[holder(use_place_holder)]
        pub product_definition: ProductDefinition,
        #[holder(use_place_holder)]
        pub documentation_ids: Vec<DocumentAny>,
    }
    impl Into<ProductDefinitionAny> for ProductDefinitionWithAssociatedDocuments {
        fn into(self) -> ProductDefinitionAny {
            ProductDefinitionAny::ProductDefinitionWithAssociatedDocuments(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_relationship)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionRelationship {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_product_definition: ProductDefinitionAny,
        #[holder(use_place_holder)]
        pub related_product_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_substitute)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionSubstitute {
        pub description: Text,
        #[holder(use_place_holder)]
        pub context_relationship: ProductDefinitionRelationship,
        #[holder(use_place_holder)]
        pub substitute_definition: ProductDefinitionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_effectivity)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionEffectivity {
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        #[holder(use_place_holder)]
        pub usage: ProductDefinitionRelationship,
    }
    impl Into<EffectivityAny> for ProductDefinitionEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::ProductDefinitionEffectivity(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = characterized_object)]
    #[holder(generate_deserialize)]
    pub struct CharacterizedObject {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinition {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub definition: CharacterizedDefinition,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_shape)]
        ProductDefinitionShape(Box<ProductDefinitionShape>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = product_definition_shape)]
    #[holder(generate_deserialize)]
    pub struct ProductDefinitionShape {
        #[holder(use_place_holder)]
        pub property_definition: PropertyDefinition,
    }
    impl Into<PropertyDefinitionAny> for ProductDefinitionShape {
        fn into(self) -> PropertyDefinitionAny {
            PropertyDefinitionAny::ProductDefinitionShape(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspect {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub of_shape: ProductDefinitionShape,
        pub product_definitional: Logical,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_aspect_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeAspectRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_shape_aspect: ShapeAspect,
        #[holder(use_place_holder)]
        pub related_shape_aspect: ShapeAspect,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentation {
        #[holder(use_place_holder)]
        pub representation: Representation,
    }
    impl Into<RepresentationAny> for ShapeRepresentation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::ShapeRepresentation(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = property_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct PropertyDefinitionRepresentation {
        #[holder(use_place_holder)]
        pub definition: PropertyDefinitionAny,
        #[holder(use_place_holder)]
        pub used_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PropertyDefinitionRepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = shape_definition_representation)]
        ShapeDefinitionRepresentation(Box<ShapeDefinitionRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct ShapeRepresentationRelationship {
        #[holder(use_place_holder)]
        pub representation_relationship: RepresentationRelationship,
    }
    impl Into<RepresentationRelationshipAny> for ShapeRepresentationRelationship {
        fn into(self) -> RepresentationRelationshipAny {
            RepresentationRelationshipAny::ShapeRepresentationRelationship(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_shape_representation)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentShapeRepresentation {
        #[holder(use_place_holder)]
        pub representation_relation: ShapeRepresentationRelationship,
        #[holder(use_place_holder)]
        pub represented_product_relation: ProductDefinitionShape,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shape_definition_representation)]
    #[holder(generate_deserialize)]
    pub struct ShapeDefinitionRepresentation {
        #[holder(use_place_holder)]
        pub property_definition_representation: PropertyDefinitionRepresentation,
    }
    impl Into<PropertyDefinitionRepresentationAny> for ShapeDefinitionRepresentation {
        fn into(self) -> PropertyDefinitionRepresentationAny {
            PropertyDefinitionRepresentationAny::ShapeDefinitionRepresentation(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = name_assignment)]
    #[holder(generate_deserialize)]
    pub struct NameAssignment {
        pub assigned_name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = external_referent_assignment)]
    #[holder(generate_deserialize)]
    pub struct ExternalReferentAssignment {
        pub assigned_name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ExternalReferentAssignmentAny {
        #[holder(use_place_holder)]
        # [holder (field = library_assignment)]
        LibraryAssignment(Box<LibraryAssignment>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = library_assignment)]
    #[holder(generate_deserialize)]
    pub struct LibraryAssignment {
        #[holder(use_place_holder)]
        pub external_referent_assignment: ExternalReferentAssignment,
        #[holder(use_place_holder)]
        pub frame_of_reference: LibraryContext,
    }
    impl Into<ExternalReferentAssignmentAny> for LibraryAssignment {
        fn into(self) -> ExternalReferentAssignmentAny {
            ExternalReferentAssignmentAny::LibraryAssignment(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_reference)]
    #[holder(generate_deserialize)]
    pub struct DocumentReference {
        #[holder(use_place_holder)]
        pub assigned_document: DocumentAny,
        pub source: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestAssignment {
        #[holder(use_place_holder)]
        pub assigned_action_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_assignment)]
    #[holder(generate_deserialize)]
    pub struct ActionAssignment {
        #[holder(use_place_holder)]
        pub assigned_action: ActionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_assignment)]
    #[holder(generate_deserialize)]
    pub struct CertificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_certification: Certification,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_assignment)]
    #[holder(generate_deserialize)]
    pub struct ApprovalAssignment {
        #[holder(use_place_holder)]
        pub assigned_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_assignment)]
    #[holder(generate_deserialize)]
    pub struct ContractAssignment {
        #[holder(use_place_holder)]
        pub assigned_contract: Contract,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_assignment)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationAssignment {
        #[holder(use_place_holder)]
        pub assigned_security_classification: SecurityClassification,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_assignment)]
    #[holder(generate_deserialize)]
    pub struct PersonAssignment {
        #[holder(use_place_holder)]
        pub assigned_person: Person,
        #[holder(use_place_holder)]
        pub role: PersonRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct OrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_organization: Organization,
        #[holder(use_place_holder)]
        pub role: OrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_assignment)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationAssignment {
        #[holder(use_place_holder)]
        pub assigned_person_and_organization: PersonAndOrganization,
        #[holder(use_place_holder)]
        pub role: PersonAndOrganizationRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_assignment)]
    #[holder(generate_deserialize)]
    pub struct DateAssignment {
        #[holder(use_place_holder)]
        pub assigned_date: DateAny,
        #[holder(use_place_holder)]
        pub role: DateRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = time_assignment)]
    #[holder(generate_deserialize)]
    pub struct TimeAssignment {
        #[holder(use_place_holder)]
        pub assigned_time: LocalTime,
        #[holder(use_place_holder)]
        pub role: TimeRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time_assignment)]
    #[holder(generate_deserialize)]
    pub struct DateAndTimeAssignment {
        #[holder(use_place_holder)]
        pub assigned_date_and_time: DateAndTime,
        #[holder(use_place_holder)]
        pub role: DateTimeRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group_assignment)]
    #[holder(generate_deserialize)]
    pub struct GroupAssignment {
        #[holder(use_place_holder)]
        pub assigned_group: Group,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = effectivity_assignment)]
    #[holder(generate_deserialize)]
    pub struct EffectivityAssignment {
        #[holder(use_place_holder)]
        pub assigned_effectivity: EffectivityAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_type)]
    #[holder(generate_deserialize)]
    pub struct DocumentType {
        pub product_data_type: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document)]
    #[holder(generate_deserialize)]
    pub struct Document {
        pub id: Identifier,
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub kind: DocumentType,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DocumentAny {
        #[holder(use_place_holder)]
        # [holder (field = document_with_class)]
        DocumentWithClass(Box<DocumentWithClass>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_with_class)]
    #[holder(generate_deserialize)]
    pub struct DocumentWithClass {
        #[holder(use_place_holder)]
        pub document: Document,
        pub class: Identifier,
    }
    impl Into<DocumentAny> for DocumentWithClass {
        fn into(self) -> DocumentAny {
            DocumentAny::DocumentWithClass(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_usage_constraint)]
    #[holder(generate_deserialize)]
    pub struct DocumentUsageConstraint {
        #[holder(use_place_holder)]
        pub source: DocumentAny,
        pub subject_element: Label,
        pub subject_element_value: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = document_relationship)]
    #[holder(generate_deserialize)]
    pub struct DocumentRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_document: DocumentAny,
        #[holder(use_place_holder)]
        pub related_document: DocumentAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action)]
    #[holder(generate_deserialize)]
    pub struct Action {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub chosen_method: ActionMethod,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ActionAny {
        #[holder(use_place_holder)]
        # [holder (field = executed_action)]
        ExecutedActionAny(Box<ExecutedActionAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = executed_action)]
    #[holder(generate_deserialize)]
    pub struct ExecutedAction {
        #[holder(use_place_holder)]
        pub action: Action,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ExecutedActionAny {
        #[holder(use_place_holder)]
        # [holder (field = directed_action)]
        DirectedAction(Box<DirectedAction>),
    }
    impl Into<ActionAny> for ExecutedActionAny {
        fn into(self) -> ActionAny {
            ActionAny::ExecutedActionAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = directed_action)]
    #[holder(generate_deserialize)]
    pub struct DirectedAction {
        #[holder(use_place_holder)]
        pub executed_action: ExecutedAction,
        #[holder(use_place_holder)]
        pub directive: ActionDirective,
    }
    impl Into<ExecutedActionAny> for DirectedAction {
        fn into(self) -> ExecutedActionAny {
            ExecutedActionAny::DirectedAction(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_status)]
    #[holder(generate_deserialize)]
    pub struct ActionStatus {
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_action: ExecutedActionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_status)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestStatus {
        pub status: Label,
        #[holder(use_place_holder)]
        pub assigned_request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_relationship)]
    #[holder(generate_deserialize)]
    pub struct ActionRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_action: ActionAny,
        #[holder(use_place_holder)]
        pub related_action: ActionAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_method)]
    #[holder(generate_deserialize)]
    pub struct ActionMethod {
        pub name: Label,
        pub description: Text,
        pub consequence: Text,
        pub purpose: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_request_solution)]
    #[holder(generate_deserialize)]
    pub struct ActionRequestSolution {
        #[holder(use_place_holder)]
        pub method: ActionMethod,
        #[holder(use_place_holder)]
        pub request: VersionedActionRequest,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_method_relationship)]
    #[holder(generate_deserialize)]
    pub struct ActionMethodRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_method: ActionMethod,
        #[holder(use_place_holder)]
        pub related_method: ActionMethod,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = versioned_action_request)]
    #[holder(generate_deserialize)]
    pub struct VersionedActionRequest {
        pub id: Identifier,
        pub version: Label,
        pub purpose: Text,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_directive)]
    #[holder(generate_deserialize)]
    pub struct ActionDirective {
        pub name: Label,
        pub description: Text,
        pub analysis: Text,
        pub comment: Text,
        #[holder(use_place_holder)]
        pub requests: Vec<VersionedActionRequest>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_resource)]
    #[holder(generate_deserialize)]
    pub struct ActionResource {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub usage: Vec<SupportedItem>,
        #[holder(use_place_holder)]
        pub kind: ActionResourceType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_resource_relationship)]
    #[holder(generate_deserialize)]
    pub struct ActionResourceRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_resource: ActionResource,
        #[holder(use_place_holder)]
        pub related_resource: ActionResource,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = action_resource_type)]
    #[holder(generate_deserialize)]
    pub struct ActionResourceType {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification_type)]
    #[holder(generate_deserialize)]
    pub struct CertificationType {
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = certification)]
    #[holder(generate_deserialize)]
    pub struct Certification {
        pub name: Label,
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: CertificationType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_status)]
    #[holder(generate_deserialize)]
    pub struct ApprovalStatus {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval)]
    #[holder(generate_deserialize)]
    pub struct Approval {
        #[holder(use_place_holder)]
        pub status: ApprovalStatus,
        pub level: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_date_time)]
    #[holder(generate_deserialize)]
    pub struct ApprovalDateTime {
        #[holder(use_place_holder)]
        pub date_time: DateTimeSelect,
        #[holder(use_place_holder)]
        pub dated_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_person_organization)]
    #[holder(generate_deserialize)]
    pub struct ApprovalPersonOrganization {
        #[holder(use_place_holder)]
        pub person_organization: PersonOrganizationSelect,
        #[holder(use_place_holder)]
        pub authorized_approval: Approval,
        #[holder(use_place_holder)]
        pub role: ApprovalRole,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_role)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRole {
        pub role: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = approval_relationship)]
    #[holder(generate_deserialize)]
    pub struct ApprovalRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_approval: Approval,
        #[holder(use_place_holder)]
        pub related_approval: Approval,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification_level)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassificationLevel {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = security_classification)]
    #[holder(generate_deserialize)]
    pub struct SecurityClassification {
        pub name: Label,
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub security_level: SecurityClassificationLevel,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract_type)]
    #[holder(generate_deserialize)]
    pub struct ContractType {
        pub description: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = contract)]
    #[holder(generate_deserialize)]
    pub struct Contract {
        pub name: Label,
        pub purpose: Text,
        #[holder(use_place_holder)]
        pub kind: ContractType,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = address)]
    #[holder(generate_deserialize)]
    pub struct Address {
        pub internal_location: Option<Label>,
        pub street_number: Option<Label>,
        pub street: Option<Label>,
        pub postal_box: Option<Label>,
        pub town: Option<Label>,
        pub region: Option<Label>,
        pub postal_code: Option<Label>,
        pub country: Option<Label>,
        pub facsimile_number: Option<Label>,
        pub telephone_number: Option<Label>,
        pub electronic_mail_address: Option<Label>,
        pub telex_number: Option<Label>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum AddressAny {
        #[holder(use_place_holder)]
        # [holder (field = personal_address)]
        PersonalAddress(Box<PersonalAddress>),
        #[holder(use_place_holder)]
        # [holder (field = organizational_address)]
        OrganizationalAddress(Box<OrganizationalAddress>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = personal_address)]
    #[holder(generate_deserialize)]
    pub struct PersonalAddress {
        #[holder(use_place_holder)]
        pub address: Address,
        #[holder(use_place_holder)]
        pub people: Vec<Person>,
        pub description: Text,
    }
    impl Into<AddressAny> for PersonalAddress {
        fn into(self) -> AddressAny {
            AddressAny::PersonalAddress(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_address)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalAddress {
        #[holder(use_place_holder)]
        pub address: Address,
        #[holder(use_place_holder)]
        pub organizations: Vec<Organization>,
        pub description: Text,
    }
    impl Into<AddressAny> for OrganizationalAddress {
        fn into(self) -> AddressAny {
            AddressAny::OrganizationalAddress(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person)]
    #[holder(generate_deserialize)]
    pub struct Person {
        pub id: Identifier,
        pub last_name: Option<Label>,
        pub first_name: Option<Label>,
        pub middle_names: Option<Vec<Label>>,
        pub prefix_titles: Option<Vec<Label>>,
        pub suffix_titles: Option<Vec<Label>>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization)]
    #[holder(generate_deserialize)]
    pub struct Organization {
        pub id: Option<Identifier>,
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organizational_project)]
    #[holder(generate_deserialize)]
    pub struct OrganizationalProject {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub responsible_organizations: Vec<Organization>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganization {
        #[holder(use_place_holder)]
        pub the_person: Person,
        #[holder(use_place_holder)]
        pub the_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_relationship)]
    #[holder(generate_deserialize)]
    pub struct OrganizationRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_organization: Organization,
        #[holder(use_place_holder)]
        pub related_organization: Organization,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_and_organization_role)]
    #[holder(generate_deserialize)]
    pub struct PersonAndOrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = person_role)]
    #[holder(generate_deserialize)]
    pub struct PersonRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = organization_role)]
    #[holder(generate_deserialize)]
    pub struct OrganizationRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date)]
    #[holder(generate_deserialize)]
    pub struct Date {
        pub year_component: YearNumber,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DateAny {
        #[holder(use_place_holder)]
        # [holder (field = calendar_date)]
        CalendarDate(Box<CalendarDate>),
        #[holder(use_place_holder)]
        # [holder (field = ordinal_date)]
        OrdinalDate(Box<OrdinalDate>),
        #[holder(use_place_holder)]
        # [holder (field = week_of_year_and_day_date)]
        WeekOfYearAndDayDate(Box<WeekOfYearAndDayDate>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = calendar_date)]
    #[holder(generate_deserialize)]
    pub struct CalendarDate {
        #[holder(use_place_holder)]
        pub date: Date,
        pub day_component: DayInMonthNumber,
        pub month_component: MonthInYearNumber,
    }
    impl Into<DateAny> for CalendarDate {
        fn into(self) -> DateAny {
            DateAny::CalendarDate(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ordinal_date)]
    #[holder(generate_deserialize)]
    pub struct OrdinalDate {
        #[holder(use_place_holder)]
        pub date: Date,
        pub day_component: DayInYearNumber,
    }
    impl Into<DateAny> for OrdinalDate {
        fn into(self) -> DateAny {
            DateAny::OrdinalDate(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = week_of_year_and_day_date)]
    #[holder(generate_deserialize)]
    pub struct WeekOfYearAndDayDate {
        #[holder(use_place_holder)]
        pub date: Date,
        pub week_component: WeekInYearNumber,
        pub day_component: Option<DayInWeekNumber>,
    }
    impl Into<DateAny> for WeekOfYearAndDayDate {
        fn into(self) -> DateAny {
            DateAny::WeekOfYearAndDayDate(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = coordinated_universal_time_offset)]
    #[holder(generate_deserialize)]
    pub struct CoordinatedUniversalTimeOffset {
        pub hour_offset: HourInDay,
        pub minute_offset: Option<MinuteInHour>,
        pub sense: AheadOrBehind,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = local_time)]
    #[holder(generate_deserialize)]
    pub struct LocalTime {
        pub hour_component: HourInDay,
        pub minute_component: Option<MinuteInHour>,
        pub second_component: Option<SecondInMinute>,
        #[holder(use_place_holder)]
        pub zone: CoordinatedUniversalTimeOffset,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_and_time)]
    #[holder(generate_deserialize)]
    pub struct DateAndTime {
        #[holder(use_place_holder)]
        pub date_component: DateAny,
        #[holder(use_place_holder)]
        pub time_component: LocalTime,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_time_role)]
    #[holder(generate_deserialize)]
    pub struct DateTimeRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = date_role)]
    #[holder(generate_deserialize)]
    pub struct DateRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = time_role)]
    #[holder(generate_deserialize)]
    pub struct TimeRole {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group)]
    #[holder(generate_deserialize)]
    pub struct Group {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = group_relationship)]
    #[holder(generate_deserialize)]
    pub struct GroupRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_group: Group,
        #[holder(use_place_holder)]
        pub related_group: Group,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = effectivity)]
    #[holder(generate_deserialize)]
    pub struct Effectivity {
        pub id: Identifier,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EffectivityAny {
        #[holder(use_place_holder)]
        # [holder (field = product_definition_effectivity)]
        ProductDefinitionEffectivity(Box<ProductDefinitionEffectivity>),
        #[holder(use_place_holder)]
        # [holder (field = serial_numbered_effectivity)]
        SerialNumberedEffectivity(Box<SerialNumberedEffectivity>),
        #[holder(use_place_holder)]
        # [holder (field = dated_effectivity)]
        DatedEffectivity(Box<DatedEffectivity>),
        #[holder(use_place_holder)]
        # [holder (field = lot_effectivity)]
        LotEffectivity(Box<LotEffectivity>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = serial_numbered_effectivity)]
    #[holder(generate_deserialize)]
    pub struct SerialNumberedEffectivity {
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        pub effectivity_start_id: Identifier,
        pub effectivity_end_id: Option<Identifier>,
    }
    impl Into<EffectivityAny> for SerialNumberedEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::SerialNumberedEffectivity(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dated_effectivity)]
    #[holder(generate_deserialize)]
    pub struct DatedEffectivity {
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        #[holder(use_place_holder)]
        pub effectivity_start_date: DateAndTime,
        #[holder(use_place_holder)]
        pub effectivity_end_date: Option<DateAndTime>,
    }
    impl Into<EffectivityAny> for DatedEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::DatedEffectivity(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = lot_effectivity)]
    #[holder(generate_deserialize)]
    pub struct LotEffectivity {
        #[holder(use_place_holder)]
        pub effectivity: Effectivity,
        pub effectivity_lot_id: Identifier,
        #[holder(use_place_holder)]
        pub effectivity_lot_size: MeasureWithUnitAny,
    }
    impl Into<EffectivityAny> for LotEffectivity {
        fn into(self) -> EffectivityAny {
            EffectivityAny::LotEffectivity(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = external_source)]
    #[holder(generate_deserialize)]
    pub struct ExternalSource {
        #[holder(use_place_holder)]
        pub source_id: SourceItem,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = external_source_relationship)]
    #[holder(generate_deserialize)]
    pub struct ExternalSourceRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub relating_source: ExternalSource,
        #[holder(use_place_holder)]
        pub related_source: ExternalSource,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pre_defined_item)]
    #[holder(generate_deserialize)]
    pub struct PreDefinedItem {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = externally_defined_item)]
    #[holder(generate_deserialize)]
    pub struct ExternallyDefinedItem {
        #[holder(use_place_holder)]
        pub item_id: SourceItem,
        #[holder(use_place_holder)]
        pub source: ExternalSource,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = named_unit)]
    #[holder(generate_deserialize)]
    pub struct NamedUnit {
        #[holder(use_place_holder)]
        pub dimensions: DimensionalExponents,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum NamedUnitAny {
        #[holder(use_place_holder)]
        # [holder (field = si_unit)]
        SiUnit(Box<SiUnit>),
        #[holder(use_place_holder)]
        # [holder (field = conversion_based_unit)]
        ConversionBasedUnit(Box<ConversionBasedUnit>),
        #[holder(use_place_holder)]
        # [holder (field = context_dependent_unit)]
        ContextDependentUnit(Box<ContextDependentUnit>),
        #[holder(use_place_holder)]
        # [holder (field = length_unit)]
        LengthUnit(Box<LengthUnit>),
        #[holder(use_place_holder)]
        # [holder (field = mass_unit)]
        MassUnit(Box<MassUnit>),
        #[holder(use_place_holder)]
        # [holder (field = time_unit)]
        TimeUnit(Box<TimeUnit>),
        #[holder(use_place_holder)]
        # [holder (field = electric_current_unit)]
        ElectricCurrentUnit(Box<ElectricCurrentUnit>),
        #[holder(use_place_holder)]
        # [holder (field = thermodynamic_temperature_unit)]
        ThermodynamicTemperatureUnit(Box<ThermodynamicTemperatureUnit>),
        #[holder(use_place_holder)]
        # [holder (field = amount_of_substance_unit)]
        AmountOfSubstanceUnit(Box<AmountOfSubstanceUnit>),
        #[holder(use_place_holder)]
        # [holder (field = luminous_intensity_unit)]
        LuminousIntensityUnit(Box<LuminousIntensityUnit>),
        #[holder(use_place_holder)]
        # [holder (field = plane_angle_unit)]
        PlaneAngleUnit(Box<PlaneAngleUnit>),
        #[holder(use_place_holder)]
        # [holder (field = solid_angle_unit)]
        SolidAngleUnit(Box<SolidAngleUnit>),
        #[holder(use_place_holder)]
        # [holder (field = area_unit)]
        AreaUnit(Box<AreaUnit>),
        #[holder(use_place_holder)]
        # [holder (field = volume_unit)]
        VolumeUnit(Box<VolumeUnit>),
        #[holder(use_place_holder)]
        # [holder (field = ratio_unit)]
        RatioUnit(Box<RatioUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = si_unit)]
    #[holder(generate_deserialize)]
    pub struct SiUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
        pub prefix: Option<SiPrefix>,
        pub name: SiUnitName,
    }
    impl Into<NamedUnitAny> for SiUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::SiUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conversion_based_unit)]
    #[holder(generate_deserialize)]
    pub struct ConversionBasedUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
        pub name: Label,
        #[holder(use_place_holder)]
        pub conversion_factor: MeasureWithUnitAny,
    }
    impl Into<NamedUnitAny> for ConversionBasedUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::ConversionBasedUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = context_dependent_unit)]
    #[holder(generate_deserialize)]
    pub struct ContextDependentUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
        pub name: Label,
    }
    impl Into<NamedUnitAny> for ContextDependentUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::ContextDependentUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for LengthUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::LengthUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mass_unit)]
    #[holder(generate_deserialize)]
    pub struct MassUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for MassUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::MassUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = time_unit)]
    #[holder(generate_deserialize)]
    pub struct TimeUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for TimeUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::TimeUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = electric_current_unit)]
    #[holder(generate_deserialize)]
    pub struct ElectricCurrentUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for ElectricCurrentUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::ElectricCurrentUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = thermodynamic_temperature_unit)]
    #[holder(generate_deserialize)]
    pub struct ThermodynamicTemperatureUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for ThermodynamicTemperatureUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::ThermodynamicTemperatureUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = amount_of_substance_unit)]
    #[holder(generate_deserialize)]
    pub struct AmountOfSubstanceUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for AmountOfSubstanceUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::AmountOfSubstanceUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = luminous_intensity_unit)]
    #[holder(generate_deserialize)]
    pub struct LuminousIntensityUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for LuminousIntensityUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::LuminousIntensityUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for PlaneAngleUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::PlaneAngleUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for SolidAngleUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::SolidAngleUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = area_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for AreaUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::AreaUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = volume_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for VolumeUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::VolumeUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ratio_unit)]
    #[holder(generate_deserialize)]
    pub struct RatioUnit {
        #[holder(use_place_holder)]
        pub named_unit: NamedUnit,
    }
    impl Into<NamedUnitAny> for RatioUnit {
        fn into(self) -> NamedUnitAny {
            NamedUnitAny::RatioUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = dimensional_exponents)]
    #[holder(generate_deserialize)]
    pub struct DimensionalExponents {
        pub length_exponent: f64,
        pub mass_exponent: f64,
        pub time_exponent: f64,
        pub electric_current_exponent: f64,
        pub thermodynamic_temperature_exponent: f64,
        pub amount_of_substance_exponent: f64,
        pub luminous_intensity_exponent: f64,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = derived_unit_element)]
    #[holder(generate_deserialize)]
    pub struct DerivedUnitElement {
        #[holder(use_place_holder)]
        pub unit: NamedUnitAny,
        pub exponent: f64,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = derived_unit)]
    #[holder(generate_deserialize)]
    pub struct DerivedUnit {
        #[holder(use_place_holder)]
        pub elements: Vec<DerivedUnitElement>,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = global_unit_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUnitAssignedContext {
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
        #[holder(use_place_holder)]
        pub units: Vec<Unit>,
    }
    impl Into<RepresentationContextAny> for GlobalUnitAssignedContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::GlobalUnitAssignedContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MeasureWithUnit {
        #[holder(use_place_holder)]
        pub value_component: MeasureValue,
        #[holder(use_place_holder)]
        pub unit_component: Unit,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum MeasureWithUnitAny {
        #[holder(use_place_holder)]
        # [holder (field = length_measure_with_unit)]
        LengthMeasureWithUnit(Box<LengthMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = mass_measure_with_unit)]
        MassMeasureWithUnit(Box<MassMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = time_measure_with_unit)]
        TimeMeasureWithUnit(Box<TimeMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = electric_current_measure_with_unit)]
        ElectricCurrentMeasureWithUnit(Box<ElectricCurrentMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = thermodynamic_temperature_measure_with_unit)]
        ThermodynamicTemperatureMeasureWithUnit(Box<ThermodynamicTemperatureMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = amount_of_substance_measure_with_unit)]
        AmountOfSubstanceMeasureWithUnit(Box<AmountOfSubstanceMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = luminous_intensity_measure_with_unit)]
        LuminousIntensityMeasureWithUnit(Box<LuminousIntensityMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = plane_angle_measure_with_unit)]
        PlaneAngleMeasureWithUnit(Box<PlaneAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = solid_angle_measure_with_unit)]
        SolidAngleMeasureWithUnit(Box<SolidAngleMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = area_measure_with_unit)]
        AreaMeasureWithUnit(Box<AreaMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = volume_measure_with_unit)]
        VolumeMeasureWithUnit(Box<VolumeMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = ratio_measure_with_unit)]
        RatioMeasureWithUnit(Box<RatioMeasureWithUnit>),
        #[holder(use_place_holder)]
        # [holder (field = uncertainty_measure_with_unit)]
        UncertaintyMeasureWithUnit(Box<UncertaintyMeasureWithUnit>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = length_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct LengthMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for LengthMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::LengthMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mass_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct MassMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for MassMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::MassMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = time_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct TimeMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for TimeMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::TimeMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = electric_current_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct ElectricCurrentMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for ElectricCurrentMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::ElectricCurrentMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = thermodynamic_temperature_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct ThermodynamicTemperatureMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for ThermodynamicTemperatureMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::ThermodynamicTemperatureMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = amount_of_substance_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct AmountOfSubstanceMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for AmountOfSubstanceMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::AmountOfSubstanceMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = luminous_intensity_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct LuminousIntensityMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for LuminousIntensityMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::LuminousIntensityMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct PlaneAngleMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for PlaneAngleMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::PlaneAngleMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_angle_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct SolidAngleMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for SolidAngleMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::SolidAngleMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = area_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct AreaMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for AreaMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::AreaMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = volume_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct VolumeMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for VolumeMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::VolumeMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ratio_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct RatioMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
    }
    impl Into<MeasureWithUnitAny> for RatioMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::RatioMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationContext {
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
        pub coordinate_space_dimension: DimensionCount,
    }
    impl Into<RepresentationContextAny> for GeometricRepresentationContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::GeometricRepresentationContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_representation_item)]
    #[holder(generate_deserialize)]
    pub struct GeometricRepresentationItem {
        #[holder(use_place_holder)]
        pub representation_item: RepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricRepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = point)]
        PointAny(Box<PointAny>),
        #[holder(use_place_holder)]
        # [holder (field = direction)]
        Direction(Box<Direction>),
        #[holder(use_place_holder)]
        # [holder (field = vector)]
        Vector(Box<Vector>),
        #[holder(use_place_holder)]
        # [holder (field = placement)]
        PlacementAny(Box<PlacementAny>),
        #[holder(use_place_holder)]
        # [holder (field = cartesian_transformation_operator)]
        CartesianTransformationOperatorAny(Box<CartesianTransformationOperatorAny>),
        #[holder(use_place_holder)]
        # [holder (field = curve)]
        CurveAny(Box<CurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = surface)]
        SurfaceAny(Box<SurfaceAny>),
        #[holder(use_place_holder)]
        # [holder (field = vertex_point)]
        VertexPoint(Box<VertexPoint>),
        #[holder(use_place_holder)]
        # [holder (field = edge_curve)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        # [holder (field = poly_loop)]
        PolyLoop(Box<PolyLoop>),
        #[holder(use_place_holder)]
        # [holder (field = face_surface)]
        FaceSurface(Box<FaceSurface>),
        #[holder(use_place_holder)]
        # [holder (field = solid_model)]
        SolidModelAny(Box<SolidModelAny>),
        #[holder(use_place_holder)]
        # [holder (field = boolean_result)]
        BooleanResult(Box<BooleanResult>),
        #[holder(use_place_holder)]
        # [holder (field = sphere)]
        Sphere(Box<Sphere>),
        #[holder(use_place_holder)]
        # [holder (field = right_circular_cone)]
        RightCircularCone(Box<RightCircularCone>),
        #[holder(use_place_holder)]
        # [holder (field = right_circular_cylinder)]
        RightCircularCylinder(Box<RightCircularCylinder>),
        #[holder(use_place_holder)]
        # [holder (field = torus)]
        Torus(Box<Torus>),
        #[holder(use_place_holder)]
        # [holder (field = block)]
        Block(Box<Block>),
        #[holder(use_place_holder)]
        # [holder (field = right_angular_wedge)]
        RightAngularWedge(Box<RightAngularWedge>),
        #[holder(use_place_holder)]
        # [holder (field = half_space_solid)]
        HalfSpaceSolidAny(Box<HalfSpaceSolidAny>),
        #[holder(use_place_holder)]
        # [holder (field = shell_based_surface_model)]
        ShellBasedSurfaceModel(Box<ShellBasedSurfaceModel>),
        #[holder(use_place_holder)]
        # [holder (field = face_based_surface_model)]
        FaceBasedSurfaceModel(Box<FaceBasedSurfaceModel>),
        #[holder(use_place_holder)]
        # [holder (field = shell_based_wireframe_model)]
        ShellBasedWireframeModel(Box<ShellBasedWireframeModel>),
        #[holder(use_place_holder)]
        # [holder (field = edge_based_wireframe_model)]
        EdgeBasedWireframeModel(Box<EdgeBasedWireframeModel>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_set)]
        GeometricSetAny(Box<GeometricSetAny>),
    }
    impl Into<RepresentationItemAny> for GeometricRepresentationItemAny {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::GeometricRepresentationItemAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point)]
    #[holder(generate_deserialize)]
    pub struct Point {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PointAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_point)]
        CartesianPointAny(Box<CartesianPointAny>),
        #[holder(use_place_holder)]
        # [holder (field = point_on_curve)]
        PointOnCurve(Box<PointOnCurve>),
        #[holder(use_place_holder)]
        # [holder (field = point_on_surface)]
        PointOnSurface(Box<PointOnSurface>),
        #[holder(use_place_holder)]
        # [holder (field = point_replica)]
        PointReplica(Box<PointReplica>),
        #[holder(use_place_holder)]
        # [holder (field = degenerate_pcurve)]
        DegeneratePcurveAny(Box<DegeneratePcurveAny>),
    }
    impl Into<GeometricRepresentationItemAny> for PointAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::PointAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_point)]
    #[holder(generate_deserialize)]
    pub struct CartesianPoint {
        #[holder(use_place_holder)]
        pub point: Point,
        pub coordinates: Vec<LengthMeasure>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CartesianPointAny {
        #[holder(use_place_holder)]
        # [holder (field = cylindrical_point)]
        CylindricalPoint(Box<CylindricalPoint>),
        #[holder(use_place_holder)]
        # [holder (field = spherical_point)]
        SphericalPoint(Box<SphericalPoint>),
        #[holder(use_place_holder)]
        # [holder (field = polar_point)]
        PolarPoint(Box<PolarPoint>),
    }
    impl Into<PointAny> for CartesianPointAny {
        fn into(self) -> PointAny {
            PointAny::CartesianPointAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cylindrical_point)]
    #[holder(generate_deserialize)]
    pub struct CylindricalPoint {
        #[holder(use_place_holder)]
        pub cartesian_point: CartesianPoint,
        pub r: LengthMeasure,
        pub theta: PlaneAngleMeasure,
        pub z: LengthMeasure,
    }
    impl Into<CartesianPointAny> for CylindricalPoint {
        fn into(self) -> CartesianPointAny {
            CartesianPointAny::CylindricalPoint(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = spherical_point)]
    #[holder(generate_deserialize)]
    pub struct SphericalPoint {
        #[holder(use_place_holder)]
        pub cartesian_point: CartesianPoint,
        pub r: LengthMeasure,
        pub theta: PlaneAngleMeasure,
        pub phi: PlaneAngleMeasure,
    }
    impl Into<CartesianPointAny> for SphericalPoint {
        fn into(self) -> CartesianPointAny {
            CartesianPointAny::SphericalPoint(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = polar_point)]
    #[holder(generate_deserialize)]
    pub struct PolarPoint {
        #[holder(use_place_holder)]
        pub cartesian_point: CartesianPoint,
        pub r: LengthMeasure,
        pub theta: PlaneAngleMeasure,
    }
    impl Into<CartesianPointAny> for PolarPoint {
        fn into(self) -> CartesianPointAny {
            CartesianPointAny::PolarPoint(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_on_curve)]
    #[holder(generate_deserialize)]
    pub struct PointOnCurve {
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        pub point_parameter: ParameterValue,
    }
    impl Into<PointAny> for PointOnCurve {
        fn into(self) -> PointAny {
            PointAny::PointOnCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_on_surface)]
    #[holder(generate_deserialize)]
    pub struct PointOnSurface {
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        pub point_parameter_u: ParameterValue,
        pub point_parameter_v: ParameterValue,
    }
    impl Into<PointAny> for PointOnSurface {
        fn into(self) -> PointAny {
            PointAny::PointOnSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = point_replica)]
    #[holder(generate_deserialize)]
    pub struct PointReplica {
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub parent_pt: PointAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    impl Into<PointAny> for PointReplica {
        fn into(self) -> PointAny {
            PointAny::PointReplica(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct DegeneratePcurve {
        #[holder(use_place_holder)]
        pub point: Point,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum DegeneratePcurveAny {
        #[holder(use_place_holder)]
        # [holder (field = evaluated_degenerate_pcurve)]
        EvaluatedDegeneratePcurve(Box<EvaluatedDegeneratePcurve>),
    }
    impl Into<PointAny> for DegeneratePcurveAny {
        fn into(self) -> PointAny {
            PointAny::DegeneratePcurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = evaluated_degenerate_pcurve)]
    #[holder(generate_deserialize)]
    pub struct EvaluatedDegeneratePcurve {
        #[holder(use_place_holder)]
        pub degenerate_pcurve: DegeneratePcurve,
        #[holder(use_place_holder)]
        pub equivalent_point: CartesianPointAny,
    }
    impl Into<DegeneratePcurveAny> for EvaluatedDegeneratePcurve {
        fn into(self) -> DegeneratePcurveAny {
            DegeneratePcurveAny::EvaluatedDegeneratePcurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = direction)]
    #[holder(generate_deserialize)]
    pub struct Direction {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        pub direction_ratios: Vec<f64>,
    }
    impl Into<GeometricRepresentationItemAny> for Direction {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Direction(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vector)]
    #[holder(generate_deserialize)]
    pub struct Vector {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub orientation: Direction,
        pub magnitude: LengthMeasure,
    }
    impl Into<GeometricRepresentationItemAny> for Vector {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Vector(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = placement)]
    #[holder(generate_deserialize)]
    pub struct Placement {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub location: CartesianPointAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PlacementAny {
        #[holder(use_place_holder)]
        # [holder (field = axis1_placement)]
        Axis1Placement(Box<Axis1Placement>),
        #[holder(use_place_holder)]
        # [holder (field = axis2_placement_2d)]
        Axis2Placement2D(Box<Axis2Placement2D>),
        #[holder(use_place_holder)]
        # [holder (field = axis2_placement_3d)]
        Axis2Placement3D(Box<Axis2Placement3D>),
    }
    impl Into<GeometricRepresentationItemAny> for PlacementAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::PlacementAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis1_placement)]
    #[holder(generate_deserialize)]
    pub struct Axis1Placement {
        #[holder(use_place_holder)]
        pub placement: Placement,
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
    }
    impl Into<PlacementAny> for Axis1Placement {
        fn into(self) -> PlacementAny {
            PlacementAny::Axis1Placement(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_2d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement2D {
        #[holder(use_place_holder)]
        pub placement: Placement,
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    impl Into<PlacementAny> for Axis2Placement2D {
        fn into(self) -> PlacementAny {
            PlacementAny::Axis2Placement2D(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = axis2_placement_3d)]
    #[holder(generate_deserialize)]
    pub struct Axis2Placement3D {
        #[holder(use_place_holder)]
        pub placement: Placement,
        #[holder(use_place_holder)]
        pub axis: Option<Direction>,
        #[holder(use_place_holder)]
        pub ref_direction: Option<Direction>,
    }
    impl Into<PlacementAny> for Axis2Placement3D {
        fn into(self) -> PlacementAny {
            PlacementAny::Axis2Placement3D(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub functionally_defined_transformation: FunctionallyDefinedTransformation,
        #[holder(use_place_holder)]
        pub axis1: Option<Direction>,
        #[holder(use_place_holder)]
        pub axis2: Option<Direction>,
        #[holder(use_place_holder)]
        pub local_origin: CartesianPointAny,
        pub scale: Option<f64>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CartesianTransformationOperatorAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_transformation_operator_3d)]
        CartesianTransformationOperator3D(Box<CartesianTransformationOperator3D>),
        #[holder(use_place_holder)]
        # [holder (field = cartesian_transformation_operator_2d)]
        CartesianTransformationOperator2D(Box<CartesianTransformationOperator2D>),
    }
    impl Into<GeometricRepresentationItemAny> for CartesianTransformationOperatorAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::CartesianTransformationOperatorAny(Box::new(self))
        }
    }
    impl Into<FunctionallyDefinedTransformationAny> for CartesianTransformationOperatorAny {
        fn into(self) -> FunctionallyDefinedTransformationAny {
            FunctionallyDefinedTransformationAny::CartesianTransformationOperatorAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator_3d)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator3D {
        #[holder(use_place_holder)]
        pub cartesian_transformation_operator: CartesianTransformationOperator,
        #[holder(use_place_holder)]
        pub axis3: Option<Direction>,
    }
    impl Into<CartesianTransformationOperatorAny> for CartesianTransformationOperator3D {
        fn into(self) -> CartesianTransformationOperatorAny {
            CartesianTransformationOperatorAny::CartesianTransformationOperator3D(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cartesian_transformation_operator_2d)]
    #[holder(generate_deserialize)]
    pub struct CartesianTransformationOperator2D {
        #[holder(use_place_holder)]
        pub cartesian_transformation_operator: CartesianTransformationOperator,
    }
    impl Into<CartesianTransformationOperatorAny> for CartesianTransformationOperator2D {
        fn into(self) -> CartesianTransformationOperatorAny {
            CartesianTransformationOperatorAny::CartesianTransformationOperator2D(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve)]
    #[holder(generate_deserialize)]
    pub struct Curve {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CurveAny {
        #[holder(use_place_holder)]
        # [holder (field = line)]
        Line(Box<Line>),
        #[holder(use_place_holder)]
        # [holder (field = conic)]
        ConicAny(Box<ConicAny>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_curve)]
        BoundedCurveAny(Box<BoundedCurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = pcurve)]
        PcurveAny(Box<PcurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = surface_curve)]
        SurfaceCurveAny(Box<SurfaceCurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = offset_curve_2d)]
        OffsetCurve2D(Box<OffsetCurve2D>),
        #[holder(use_place_holder)]
        # [holder (field = offset_curve_3d)]
        OffsetCurve3D(Box<OffsetCurve3D>),
        #[holder(use_place_holder)]
        # [holder (field = curve_replica)]
        CurveReplica(Box<CurveReplica>),
    }
    impl Into<GeometricRepresentationItemAny> for CurveAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::CurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = line)]
    #[holder(generate_deserialize)]
    pub struct Line {
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub pnt: CartesianPointAny,
        #[holder(use_place_holder)]
        pub dir: Vector,
    }
    impl Into<CurveAny> for Line {
        fn into(self) -> CurveAny {
            CurveAny::Line(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conic)]
    #[holder(generate_deserialize)]
    pub struct Conic {
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub position: Axis2Placement,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConicAny {
        #[holder(use_place_holder)]
        # [holder (field = circle)]
        Circle(Box<Circle>),
        #[holder(use_place_holder)]
        # [holder (field = ellipse)]
        Ellipse(Box<Ellipse>),
        #[holder(use_place_holder)]
        # [holder (field = hyperbola)]
        Hyperbola(Box<Hyperbola>),
        #[holder(use_place_holder)]
        # [holder (field = parabola)]
        Parabola(Box<Parabola>),
    }
    impl Into<CurveAny> for ConicAny {
        fn into(self) -> CurveAny {
            CurveAny::ConicAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = circle)]
    #[holder(generate_deserialize)]
    pub struct Circle {
        #[holder(use_place_holder)]
        pub conic: Conic,
        pub radius: PositiveLengthMeasure,
    }
    impl Into<ConicAny> for Circle {
        fn into(self) -> ConicAny {
            ConicAny::Circle(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = ellipse)]
    #[holder(generate_deserialize)]
    pub struct Ellipse {
        #[holder(use_place_holder)]
        pub conic: Conic,
        pub semi_axis_1: PositiveLengthMeasure,
        pub semi_axis_2: PositiveLengthMeasure,
    }
    impl Into<ConicAny> for Ellipse {
        fn into(self) -> ConicAny {
            ConicAny::Ellipse(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = hyperbola)]
    #[holder(generate_deserialize)]
    pub struct Hyperbola {
        #[holder(use_place_holder)]
        pub conic: Conic,
        pub semi_axis: PositiveLengthMeasure,
        pub semi_imag_axis: PositiveLengthMeasure,
    }
    impl Into<ConicAny> for Hyperbola {
        fn into(self) -> ConicAny {
            ConicAny::Hyperbola(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = parabola)]
    #[holder(generate_deserialize)]
    pub struct Parabola {
        #[holder(use_place_holder)]
        pub conic: Conic,
        pub focal_dist: LengthMeasure,
    }
    impl Into<ConicAny> for Parabola {
        fn into(self) -> ConicAny {
            ConicAny::Parabola(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedCurve {
        #[holder(use_place_holder)]
        pub curve: Curve,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = polyline)]
        Polyline(Box<Polyline>),
        #[holder(use_place_holder)]
        # [holder (field = b_spline_curve)]
        BSplineCurveAny(Box<BSplineCurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = trimmed_curve)]
        TrimmedCurve(Box<TrimmedCurve>),
        #[holder(use_place_holder)]
        # [holder (field = composite_curve)]
        CompositeCurveAny(Box<CompositeCurveAny>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_pcurve)]
        BoundedPcurve(Box<BoundedPcurve>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_surface_curve)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
    }
    impl Into<CurveAny> for BoundedCurveAny {
        fn into(self) -> CurveAny {
            CurveAny::BoundedCurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = polyline)]
    #[holder(generate_deserialize)]
    pub struct Polyline {
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        #[holder(use_place_holder)]
        pub points: Vec<CartesianPointAny>,
    }
    impl Into<BoundedCurveAny> for Polyline {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::Polyline(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurve {
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        pub degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<CartesianPointAny>,
        pub curve_form: BSplineCurveForm,
        pub closed_curve: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BSplineCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_curve_with_knots)]
        BSplineCurveWithKnots(Box<BSplineCurveWithKnots>),
        #[holder(use_place_holder)]
        # [holder (field = uniform_curve)]
        UniformCurve(Box<UniformCurve>),
        #[holder(use_place_holder)]
        # [holder (field = quasi_uniform_curve)]
        QuasiUniformCurve(Box<QuasiUniformCurve>),
        #[holder(use_place_holder)]
        # [holder (field = bezier_curve)]
        BezierCurve(Box<BezierCurve>),
        #[holder(use_place_holder)]
        # [holder (field = rational_b_spline_curve)]
        RationalBSplineCurve(Box<RationalBSplineCurve>),
    }
    impl Into<BoundedCurveAny> for BSplineCurveAny {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::BSplineCurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_curve_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineCurveWithKnots {
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
        pub knot_multiplicities: Vec<i64>,
        pub knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    impl Into<BSplineCurveAny> for BSplineCurveWithKnots {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::BSplineCurveWithKnots(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct UniformCurve {
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
    }
    impl Into<BSplineCurveAny> for UniformCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::UniformCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_curve)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformCurve {
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
    }
    impl Into<BSplineCurveAny> for QuasiUniformCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::QuasiUniformCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bezier_curve)]
    #[holder(generate_deserialize)]
    pub struct BezierCurve {
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
    }
    impl Into<BSplineCurveAny> for BezierCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::BezierCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_curve)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineCurve {
        #[holder(use_place_holder)]
        pub b_spline_curve: BSplineCurve,
        pub weights_data: Vec<f64>,
    }
    impl Into<BSplineCurveAny> for RationalBSplineCurve {
        fn into(self) -> BSplineCurveAny {
            BSplineCurveAny::RationalBSplineCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = trimmed_curve)]
    #[holder(generate_deserialize)]
    pub struct TrimmedCurve {
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        #[holder(use_place_holder)]
        pub trim_1: Vec<TrimmingSelect>,
        #[holder(use_place_holder)]
        pub trim_2: Vec<TrimmingSelect>,
        pub sense_agreement: bool,
        pub master_representation: TrimmingPreference,
    }
    impl Into<BoundedCurveAny> for TrimmedCurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::TrimmedCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurve {
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
        #[holder(use_place_holder)]
        pub segments: Vec<CompositeCurveSegmentAny>,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = composite_curve_on_surface)]
        CompositeCurveOnSurfaceAny(Box<CompositeCurveOnSurfaceAny>),
    }
    impl Into<BoundedCurveAny> for CompositeCurveAny {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::CompositeCurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveSegment {
        #[holder(use_place_holder)]
        pub founded_item: FoundedItem,
        pub transition: TransitionCode,
        pub same_sense: bool,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveSegmentAny {
        #[holder(use_place_holder)]
        # [holder (field = reparametrised_composite_curve_segment)]
        ReparametrisedCompositeCurveSegment(Box<ReparametrisedCompositeCurveSegment>),
    }
    impl Into<FoundedItemAny> for CompositeCurveSegmentAny {
        fn into(self) -> FoundedItemAny {
            FoundedItemAny::CompositeCurveSegmentAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = reparametrised_composite_curve_segment)]
    #[holder(generate_deserialize)]
    pub struct ReparametrisedCompositeCurveSegment {
        #[holder(use_place_holder)]
        pub composite_curve_segment: CompositeCurveSegment,
        pub param_length: ParameterValue,
    }
    impl Into<CompositeCurveSegmentAny> for ReparametrisedCompositeCurveSegment {
        fn into(self) -> CompositeCurveSegmentAny {
            CompositeCurveSegmentAny::ReparametrisedCompositeCurveSegment(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = pcurve)]
    #[holder(generate_deserialize)]
    pub struct Pcurve {
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub reference_to_curve: DefinitionalRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PcurveAny {
        #[holder(use_place_holder)]
        # [holder (field = bounded_pcurve)]
        BoundedPcurve(Box<BoundedPcurve>),
    }
    impl Into<CurveAny> for PcurveAny {
        fn into(self) -> CurveAny {
            CurveAny::PcurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_pcurve)]
    #[holder(generate_deserialize)]
    pub struct BoundedPcurve {
        #[holder(use_place_holder)]
        pub pcurve: Pcurve,
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
    }
    impl Into<PcurveAny> for BoundedPcurve {
        fn into(self) -> PcurveAny {
            PcurveAny::BoundedPcurve(Box::new(self))
        }
    }
    impl Into<BoundedCurveAny> for BoundedPcurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::BoundedPcurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_curve)]
    #[holder(generate_deserialize)]
    pub struct SurfaceCurve {
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub curve_3d: CurveAny,
        #[holder(use_place_holder)]
        pub associated_geometry: Vec<PcurveOrSurface>,
        pub master_representation: PreferredSurfaceCurveRepresentation,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = intersection_curve)]
        IntersectionCurve(Box<IntersectionCurve>),
        #[holder(use_place_holder)]
        # [holder (field = seam_curve)]
        SeamCurve(Box<SeamCurve>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_surface_curve)]
        BoundedSurfaceCurve(Box<BoundedSurfaceCurve>),
    }
    impl Into<CurveAny> for SurfaceCurveAny {
        fn into(self) -> CurveAny {
            CurveAny::SurfaceCurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = intersection_curve)]
    #[holder(generate_deserialize)]
    pub struct IntersectionCurve {
        #[holder(use_place_holder)]
        pub surface_curve: SurfaceCurve,
    }
    impl Into<SurfaceCurveAny> for IntersectionCurve {
        fn into(self) -> SurfaceCurveAny {
            SurfaceCurveAny::IntersectionCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = seam_curve)]
    #[holder(generate_deserialize)]
    pub struct SeamCurve {
        #[holder(use_place_holder)]
        pub surface_curve: SurfaceCurve,
    }
    impl Into<SurfaceCurveAny> for SeamCurve {
        fn into(self) -> SurfaceCurveAny {
            SurfaceCurveAny::SeamCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurfaceCurve {
        #[holder(use_place_holder)]
        pub surface_curve: SurfaceCurve,
        #[holder(use_place_holder)]
        pub bounded_curve: BoundedCurve,
    }
    impl Into<SurfaceCurveAny> for BoundedSurfaceCurve {
        fn into(self) -> SurfaceCurveAny {
            SurfaceCurveAny::BoundedSurfaceCurve(Box::new(self))
        }
    }
    impl Into<BoundedCurveAny> for BoundedSurfaceCurve {
        fn into(self) -> BoundedCurveAny {
            BoundedCurveAny::BoundedSurfaceCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = composite_curve_on_surface)]
    #[holder(generate_deserialize)]
    pub struct CompositeCurveOnSurface {
        #[holder(use_place_holder)]
        pub composite_curve: CompositeCurve,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum CompositeCurveOnSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = boundary_curve)]
        BoundaryCurveAny(Box<BoundaryCurveAny>),
    }
    impl Into<CompositeCurveAny> for CompositeCurveOnSurfaceAny {
        fn into(self) -> CompositeCurveAny {
            CompositeCurveAny::CompositeCurveOnSurfaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_curve_2d)]
    #[holder(generate_deserialize)]
    pub struct OffsetCurve2D {
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    impl Into<CurveAny> for OffsetCurve2D {
        fn into(self) -> CurveAny {
            CurveAny::OffsetCurve2D(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_curve_3d)]
    #[holder(generate_deserialize)]
    pub struct OffsetCurve3D {
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub basis_curve: CurveAny,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
        #[holder(use_place_holder)]
        pub ref_direction: Direction,
    }
    impl Into<CurveAny> for OffsetCurve3D {
        fn into(self) -> CurveAny {
            CurveAny::OffsetCurve3D(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_replica)]
    #[holder(generate_deserialize)]
    pub struct CurveReplica {
        #[holder(use_place_holder)]
        pub curve: Curve,
        #[holder(use_place_holder)]
        pub parent_curve: CurveAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    impl Into<CurveAny> for CurveReplica {
        fn into(self) -> CurveAny {
            CurveAny::CurveReplica(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface)]
    #[holder(generate_deserialize)]
    pub struct Surface {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = elementary_surface)]
        ElementarySurfaceAny(Box<ElementarySurfaceAny>),
        #[holder(use_place_holder)]
        # [holder (field = swept_surface)]
        SweptSurfaceAny(Box<SweptSurfaceAny>),
        #[holder(use_place_holder)]
        # [holder (field = bounded_surface)]
        BoundedSurfaceAny(Box<BoundedSurfaceAny>),
        #[holder(use_place_holder)]
        # [holder (field = offset_surface)]
        OffsetSurface(Box<OffsetSurface>),
        #[holder(use_place_holder)]
        # [holder (field = surface_replica)]
        SurfaceReplica(Box<SurfaceReplica>),
    }
    impl Into<GeometricRepresentationItemAny> for SurfaceAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::SurfaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = elementary_surface)]
    #[holder(generate_deserialize)]
    pub struct ElementarySurface {
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ElementarySurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = plane)]
        Plane(Box<Plane>),
        #[holder(use_place_holder)]
        # [holder (field = cylindrical_surface)]
        CylindricalSurface(Box<CylindricalSurface>),
        #[holder(use_place_holder)]
        # [holder (field = conical_surface)]
        ConicalSurface(Box<ConicalSurface>),
        #[holder(use_place_holder)]
        # [holder (field = spherical_surface)]
        SphericalSurface(Box<SphericalSurface>),
        #[holder(use_place_holder)]
        # [holder (field = toroidal_surface)]
        ToroidalSurfaceAny(Box<ToroidalSurfaceAny>),
    }
    impl Into<SurfaceAny> for ElementarySurfaceAny {
        fn into(self) -> SurfaceAny {
            SurfaceAny::ElementarySurfaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = plane)]
    #[holder(generate_deserialize)]
    pub struct Plane {
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
    }
    impl Into<ElementarySurfaceAny> for Plane {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::Plane(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = cylindrical_surface)]
    #[holder(generate_deserialize)]
    pub struct CylindricalSurface {
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        pub radius: PositiveLengthMeasure,
    }
    impl Into<ElementarySurfaceAny> for CylindricalSurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::CylindricalSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = conical_surface)]
    #[holder(generate_deserialize)]
    pub struct ConicalSurface {
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        pub radius: LengthMeasure,
        pub semi_angle: PlaneAngleMeasure,
    }
    impl Into<ElementarySurfaceAny> for ConicalSurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::ConicalSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = spherical_surface)]
    #[holder(generate_deserialize)]
    pub struct SphericalSurface {
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        pub radius: PositiveLengthMeasure,
    }
    impl Into<ElementarySurfaceAny> for SphericalSurface {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::SphericalSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct ToroidalSurface {
        #[holder(use_place_holder)]
        pub elementary_surface: ElementarySurface,
        pub major_radius: PositiveLengthMeasure,
        pub minor_radius: PositiveLengthMeasure,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ToroidalSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = degenerate_toroidal_surface)]
        DegenerateToroidalSurface(Box<DegenerateToroidalSurface>),
    }
    impl Into<ElementarySurfaceAny> for ToroidalSurfaceAny {
        fn into(self) -> ElementarySurfaceAny {
            ElementarySurfaceAny::ToroidalSurfaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = degenerate_toroidal_surface)]
    #[holder(generate_deserialize)]
    pub struct DegenerateToroidalSurface {
        #[holder(use_place_holder)]
        pub toroidal_surface: ToroidalSurface,
        pub select_outer: bool,
    }
    impl Into<ToroidalSurfaceAny> for DegenerateToroidalSurface {
        fn into(self) -> ToroidalSurfaceAny {
            ToroidalSurfaceAny::DegenerateToroidalSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = swept_surface)]
    #[holder(generate_deserialize)]
    pub struct SweptSurface {
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub swept_curve: CurveAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SweptSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = surface_of_linear_extrusion)]
        SurfaceOfLinearExtrusion(Box<SurfaceOfLinearExtrusion>),
        #[holder(use_place_holder)]
        # [holder (field = surface_of_revolution)]
        SurfaceOfRevolution(Box<SurfaceOfRevolution>),
    }
    impl Into<SurfaceAny> for SweptSurfaceAny {
        fn into(self) -> SurfaceAny {
            SurfaceAny::SweptSurfaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_of_linear_extrusion)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfLinearExtrusion {
        #[holder(use_place_holder)]
        pub swept_surface: SweptSurface,
        #[holder(use_place_holder)]
        pub extrusion_axis: Vector,
    }
    impl Into<SweptSurfaceAny> for SurfaceOfLinearExtrusion {
        fn into(self) -> SweptSurfaceAny {
            SweptSurfaceAny::SurfaceOfLinearExtrusion(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_of_revolution)]
    #[holder(generate_deserialize)]
    pub struct SurfaceOfRevolution {
        #[holder(use_place_holder)]
        pub swept_surface: SweptSurface,
        #[holder(use_place_holder)]
        pub axis_position: Axis1Placement,
    }
    impl Into<SweptSurfaceAny> for SurfaceOfRevolution {
        fn into(self) -> SweptSurfaceAny {
            SweptSurfaceAny::SurfaceOfRevolution(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct BoundedSurface {
        #[holder(use_place_holder)]
        pub surface: Surface,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundedSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_surface)]
        BSplineSurfaceAny(Box<BSplineSurfaceAny>),
        #[holder(use_place_holder)]
        # [holder (field = rectangular_trimmed_surface)]
        RectangularTrimmedSurface(Box<RectangularTrimmedSurface>),
        #[holder(use_place_holder)]
        # [holder (field = curve_bounded_surface)]
        CurveBoundedSurface(Box<CurveBoundedSurface>),
        #[holder(use_place_holder)]
        # [holder (field = rectangular_composite_surface)]
        RectangularCompositeSurface(Box<RectangularCompositeSurface>),
    }
    impl Into<SurfaceAny> for BoundedSurfaceAny {
        fn into(self) -> SurfaceAny {
            SurfaceAny::BoundedSurfaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurface {
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        pub u_degree: i64,
        pub v_degree: i64,
        #[holder(use_place_holder)]
        pub control_points_list: Vec<Vec<CartesianPointAny>>,
        pub surface_form: BSplineSurfaceForm,
        pub u_closed: Logical,
        pub v_closed: Logical,
        pub self_intersect: Logical,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BSplineSurfaceAny {
        #[holder(use_place_holder)]
        # [holder (field = b_spline_surface_with_knots)]
        BSplineSurfaceWithKnots(Box<BSplineSurfaceWithKnots>),
        #[holder(use_place_holder)]
        # [holder (field = uniform_surface)]
        UniformSurface(Box<UniformSurface>),
        #[holder(use_place_holder)]
        # [holder (field = quasi_uniform_surface)]
        QuasiUniformSurface(Box<QuasiUniformSurface>),
        #[holder(use_place_holder)]
        # [holder (field = bezier_surface)]
        BezierSurface(Box<BezierSurface>),
        #[holder(use_place_holder)]
        # [holder (field = rational_b_spline_surface)]
        RationalBSplineSurface(Box<RationalBSplineSurface>),
    }
    impl Into<BoundedSurfaceAny> for BSplineSurfaceAny {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::BSplineSurfaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = b_spline_surface_with_knots)]
    #[holder(generate_deserialize)]
    pub struct BSplineSurfaceWithKnots {
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
        pub u_multiplicities: Vec<i64>,
        pub v_multiplicities: Vec<i64>,
        pub u_knots: Vec<ParameterValue>,
        pub v_knots: Vec<ParameterValue>,
        pub knot_spec: KnotType,
    }
    impl Into<BSplineSurfaceAny> for BSplineSurfaceWithKnots {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::BSplineSurfaceWithKnots(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct UniformSurface {
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
    }
    impl Into<BSplineSurfaceAny> for UniformSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::UniformSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = quasi_uniform_surface)]
    #[holder(generate_deserialize)]
    pub struct QuasiUniformSurface {
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
    }
    impl Into<BSplineSurfaceAny> for QuasiUniformSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::QuasiUniformSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = bezier_surface)]
    #[holder(generate_deserialize)]
    pub struct BezierSurface {
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
    }
    impl Into<BSplineSurfaceAny> for BezierSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::BezierSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rational_b_spline_surface)]
    #[holder(generate_deserialize)]
    pub struct RationalBSplineSurface {
        #[holder(use_place_holder)]
        pub b_spline_surface: BSplineSurface,
        pub weights_data: Vec<Vec<f64>>,
    }
    impl Into<BSplineSurfaceAny> for RationalBSplineSurface {
        fn into(self) -> BSplineSurfaceAny {
            BSplineSurfaceAny::RationalBSplineSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rectangular_trimmed_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularTrimmedSurface {
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        pub u1: ParameterValue,
        pub u2: ParameterValue,
        pub v1: ParameterValue,
        pub v2: ParameterValue,
        pub usense: bool,
        pub vsense: bool,
    }
    impl Into<BoundedSurfaceAny> for RectangularTrimmedSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::RectangularTrimmedSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = curve_bounded_surface)]
    #[holder(generate_deserialize)]
    pub struct CurveBoundedSurface {
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub boundaries: Vec<BoundaryCurveAny>,
        pub implicit_outer: bool,
    }
    impl Into<BoundedSurfaceAny> for CurveBoundedSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::CurveBoundedSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct BoundaryCurve {
        #[holder(use_place_holder)]
        pub composite_curve_on_surface: CompositeCurveOnSurface,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum BoundaryCurveAny {
        #[holder(use_place_holder)]
        # [holder (field = outer_boundary_curve)]
        OuterBoundaryCurve(Box<OuterBoundaryCurve>),
    }
    impl Into<CompositeCurveOnSurfaceAny> for BoundaryCurveAny {
        fn into(self) -> CompositeCurveOnSurfaceAny {
            CompositeCurveOnSurfaceAny::BoundaryCurveAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = outer_boundary_curve)]
    #[holder(generate_deserialize)]
    pub struct OuterBoundaryCurve {
        #[holder(use_place_holder)]
        pub boundary_curve: BoundaryCurve,
    }
    impl Into<BoundaryCurveAny> for OuterBoundaryCurve {
        fn into(self) -> BoundaryCurveAny {
            BoundaryCurveAny::OuterBoundaryCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = rectangular_composite_surface)]
    #[holder(generate_deserialize)]
    pub struct RectangularCompositeSurface {
        #[holder(use_place_holder)]
        pub bounded_surface: BoundedSurface,
        #[holder(use_place_holder)]
        pub segments: Vec<Vec<SurfacePatch>>,
    }
    impl Into<BoundedSurfaceAny> for RectangularCompositeSurface {
        fn into(self) -> BoundedSurfaceAny {
            BoundedSurfaceAny::RectangularCompositeSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_patch)]
    #[holder(generate_deserialize)]
    pub struct SurfacePatch {
        #[holder(use_place_holder)]
        pub founded_item: FoundedItem,
        #[holder(use_place_holder)]
        pub parent_surface: BoundedSurfaceAny,
        pub u_transition: TransitionCode,
        pub v_transition: TransitionCode,
        pub u_sense: bool,
        pub v_sense: bool,
    }
    impl Into<FoundedItemAny> for SurfacePatch {
        fn into(self) -> FoundedItemAny {
            FoundedItemAny::SurfacePatch(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = offset_surface)]
    #[holder(generate_deserialize)]
    pub struct OffsetSurface {
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub basis_surface: SurfaceAny,
        pub distance: LengthMeasure,
        pub self_intersect: Logical,
    }
    impl Into<SurfaceAny> for OffsetSurface {
        fn into(self) -> SurfaceAny {
            SurfaceAny::OffsetSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = surface_replica)]
    #[holder(generate_deserialize)]
    pub struct SurfaceReplica {
        #[holder(use_place_holder)]
        pub surface: Surface,
        #[holder(use_place_holder)]
        pub parent_surface: SurfaceAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperator3D,
    }
    impl Into<SurfaceAny> for SurfaceReplica {
        fn into(self) -> SurfaceAny {
            SurfaceAny::SurfaceReplica(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = topological_representation_item)]
    #[holder(generate_deserialize)]
    pub struct TopologicalRepresentationItem {
        #[holder(use_place_holder)]
        pub representation_item: RepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum TopologicalRepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = vertex)]
        VertexAny(Box<VertexAny>),
        #[holder(use_place_holder)]
        # [holder (field = edge)]
        EdgeAny(Box<EdgeAny>),
        #[holder(use_place_holder)]
        # [holder (field = path)]
        PathAny(Box<PathAny>),
        #[holder(use_place_holder)]
        # [holder (field = r#loop)]
        LoopAny(Box<LoopAny>),
        #[holder(use_place_holder)]
        # [holder (field = face_bound)]
        FaceBoundAny(Box<FaceBoundAny>),
        #[holder(use_place_holder)]
        # [holder (field = face)]
        FaceAny(Box<FaceAny>),
        #[holder(use_place_holder)]
        # [holder (field = connected_face_set)]
        ConnectedFaceSetAny(Box<ConnectedFaceSetAny>),
        #[holder(use_place_holder)]
        # [holder (field = vertex_shell)]
        VertexShell(Box<VertexShell>),
        #[holder(use_place_holder)]
        # [holder (field = wire_shell)]
        WireShell(Box<WireShell>),
        #[holder(use_place_holder)]
        # [holder (field = connected_edge_set)]
        ConnectedEdgeSet(Box<ConnectedEdgeSet>),
    }
    impl Into<RepresentationItemAny> for TopologicalRepresentationItemAny {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::TopologicalRepresentationItemAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex)]
    #[holder(generate_deserialize)]
    pub struct Vertex {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum VertexAny {
        #[holder(use_place_holder)]
        # [holder (field = vertex_point)]
        VertexPoint(Box<VertexPoint>),
    }
    impl Into<TopologicalRepresentationItemAny> for VertexAny {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::VertexAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_point)]
    #[holder(generate_deserialize)]
    pub struct VertexPoint {
        #[holder(use_place_holder)]
        pub vertex: Vertex,
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub vertex_geometry: PointAny,
    }
    impl Into<VertexAny> for VertexPoint {
        fn into(self) -> VertexAny {
            VertexAny::VertexPoint(Box::new(self))
        }
    }
    impl Into<GeometricRepresentationItemAny> for VertexPoint {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::VertexPoint(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge)]
    #[holder(generate_deserialize)]
    pub struct Edge {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub edge_start: VertexAny,
        #[holder(use_place_holder)]
        pub edge_end: VertexAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum EdgeAny {
        #[holder(use_place_holder)]
        # [holder (field = edge_curve)]
        EdgeCurve(Box<EdgeCurve>),
        #[holder(use_place_holder)]
        # [holder (field = oriented_edge)]
        OrientedEdge(Box<OrientedEdge>),
    }
    impl Into<TopologicalRepresentationItemAny> for EdgeAny {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::EdgeAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_curve)]
    #[holder(generate_deserialize)]
    pub struct EdgeCurve {
        #[holder(use_place_holder)]
        pub edge: Edge,
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub edge_geometry: CurveAny,
        pub same_sense: bool,
    }
    impl Into<EdgeAny> for EdgeCurve {
        fn into(self) -> EdgeAny {
            EdgeAny::EdgeCurve(Box::new(self))
        }
    }
    impl Into<GeometricRepresentationItemAny> for EdgeCurve {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::EdgeCurve(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_edge)]
    #[holder(generate_deserialize)]
    pub struct OrientedEdge {
        #[holder(use_place_holder)]
        pub edge: Edge,
        #[holder(use_place_holder)]
        pub edge_element: EdgeAny,
        pub orientation: bool,
    }
    impl Into<EdgeAny> for OrientedEdge {
        fn into(self) -> EdgeAny {
            EdgeAny::OrientedEdge(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = path)]
    #[holder(generate_deserialize)]
    pub struct Path {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub edge_list: Vec<OrientedEdge>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum PathAny {
        #[holder(use_place_holder)]
        # [holder (field = oriented_path)]
        OrientedPath(Box<OrientedPath>),
        #[holder(use_place_holder)]
        # [holder (field = open_path)]
        OpenPath(Box<OpenPath>),
        #[holder(use_place_holder)]
        # [holder (field = edge_loop)]
        EdgeLoop(Box<EdgeLoop>),
    }
    impl Into<TopologicalRepresentationItemAny> for PathAny {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::PathAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_path)]
    #[holder(generate_deserialize)]
    pub struct OrientedPath {
        #[holder(use_place_holder)]
        pub path: Path,
        #[holder(use_place_holder)]
        pub path_element: PathAny,
        pub orientation: bool,
    }
    impl Into<PathAny> for OrientedPath {
        fn into(self) -> PathAny {
            PathAny::OrientedPath(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = open_path)]
    #[holder(generate_deserialize)]
    pub struct OpenPath {
        #[holder(use_place_holder)]
        pub path: Path,
    }
    impl Into<PathAny> for OpenPath {
        fn into(self) -> PathAny {
            PathAny::OpenPath(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = r#loop)]
    #[holder(generate_deserialize)]
    pub struct Loop {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum LoopAny {
        #[holder(use_place_holder)]
        # [holder (field = vertex_loop)]
        VertexLoop(Box<VertexLoop>),
        #[holder(use_place_holder)]
        # [holder (field = edge_loop)]
        EdgeLoop(Box<EdgeLoop>),
        #[holder(use_place_holder)]
        # [holder (field = poly_loop)]
        PolyLoop(Box<PolyLoop>),
    }
    impl Into<TopologicalRepresentationItemAny> for LoopAny {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::LoopAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_loop)]
    #[holder(generate_deserialize)]
    pub struct VertexLoop {
        #[holder(use_place_holder)]
        pub r#loop: Loop,
        #[holder(use_place_holder)]
        pub loop_vertex: VertexAny,
    }
    impl Into<LoopAny> for VertexLoop {
        fn into(self) -> LoopAny {
            LoopAny::VertexLoop(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_loop)]
    #[holder(generate_deserialize)]
    pub struct EdgeLoop {
        #[holder(use_place_holder)]
        pub r#loop: Loop,
        #[holder(use_place_holder)]
        pub path: Path,
    }
    impl Into<LoopAny> for EdgeLoop {
        fn into(self) -> LoopAny {
            LoopAny::EdgeLoop(Box::new(self))
        }
    }
    impl Into<PathAny> for EdgeLoop {
        fn into(self) -> PathAny {
            PathAny::EdgeLoop(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = poly_loop)]
    #[holder(generate_deserialize)]
    pub struct PolyLoop {
        #[holder(use_place_holder)]
        pub r#loop: Loop,
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub polygon: Vec<CartesianPointAny>,
    }
    impl Into<LoopAny> for PolyLoop {
        fn into(self) -> LoopAny {
            LoopAny::PolyLoop(Box::new(self))
        }
    }
    impl Into<GeometricRepresentationItemAny> for PolyLoop {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::PolyLoop(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceBound {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub bound: LoopAny,
        pub orientation: bool,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceBoundAny {
        #[holder(use_place_holder)]
        # [holder (field = face_outer_bound)]
        FaceOuterBound(Box<FaceOuterBound>),
    }
    impl Into<TopologicalRepresentationItemAny> for FaceBoundAny {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::FaceBoundAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_outer_bound)]
    #[holder(generate_deserialize)]
    pub struct FaceOuterBound {
        #[holder(use_place_holder)]
        pub face_bound: FaceBound,
    }
    impl Into<FaceBoundAny> for FaceOuterBound {
        fn into(self) -> FaceBoundAny {
            FaceBoundAny::FaceOuterBound(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face)]
    #[holder(generate_deserialize)]
    pub struct Face {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub bounds: Vec<FaceBoundAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FaceAny {
        #[holder(use_place_holder)]
        # [holder (field = face_surface)]
        FaceSurface(Box<FaceSurface>),
        #[holder(use_place_holder)]
        # [holder (field = oriented_face)]
        OrientedFace(Box<OrientedFace>),
        #[holder(use_place_holder)]
        # [holder (field = subface)]
        Subface(Box<Subface>),
    }
    impl Into<TopologicalRepresentationItemAny> for FaceAny {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::FaceAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_surface)]
    #[holder(generate_deserialize)]
    pub struct FaceSurface {
        #[holder(use_place_holder)]
        pub face: Face,
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub face_geometry: SurfaceAny,
        pub same_sense: bool,
    }
    impl Into<FaceAny> for FaceSurface {
        fn into(self) -> FaceAny {
            FaceAny::FaceSurface(Box::new(self))
        }
    }
    impl Into<GeometricRepresentationItemAny> for FaceSurface {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::FaceSurface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_face)]
    #[holder(generate_deserialize)]
    pub struct OrientedFace {
        #[holder(use_place_holder)]
        pub face: Face,
        #[holder(use_place_holder)]
        pub face_element: FaceAny,
        pub orientation: bool,
    }
    impl Into<FaceAny> for OrientedFace {
        fn into(self) -> FaceAny {
            FaceAny::OrientedFace(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = subface)]
    #[holder(generate_deserialize)]
    pub struct Subface {
        #[holder(use_place_holder)]
        pub face: Face,
        #[holder(use_place_holder)]
        pub parent_face: FaceAny,
    }
    impl Into<FaceAny> for Subface {
        fn into(self) -> FaceAny {
            FaceAny::Subface(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = connected_face_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedFaceSet {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub cfs_faces: Vec<FaceAny>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ConnectedFaceSetAny {
        #[holder(use_place_holder)]
        # [holder (field = open_shell)]
        OpenShellAny(Box<OpenShellAny>),
        #[holder(use_place_holder)]
        # [holder (field = closed_shell)]
        ClosedShellAny(Box<ClosedShellAny>),
    }
    impl Into<TopologicalRepresentationItemAny> for ConnectedFaceSetAny {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::ConnectedFaceSetAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = vertex_shell)]
    #[holder(generate_deserialize)]
    pub struct VertexShell {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub vertex_shell_extent: VertexLoop,
    }
    impl Into<TopologicalRepresentationItemAny> for VertexShell {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::VertexShell(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = wire_shell)]
    #[holder(generate_deserialize)]
    pub struct WireShell {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub wire_shell_extent: Vec<LoopAny>,
    }
    impl Into<TopologicalRepresentationItemAny> for WireShell {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::WireShell(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = open_shell)]
    #[holder(generate_deserialize)]
    pub struct OpenShell {
        #[holder(use_place_holder)]
        pub connected_face_set: ConnectedFaceSet,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum OpenShellAny {
        #[holder(use_place_holder)]
        # [holder (field = oriented_open_shell)]
        OrientedOpenShell(Box<OrientedOpenShell>),
    }
    impl Into<ConnectedFaceSetAny> for OpenShellAny {
        fn into(self) -> ConnectedFaceSetAny {
            ConnectedFaceSetAny::OpenShellAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_open_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedOpenShell {
        #[holder(use_place_holder)]
        pub open_shell: OpenShell,
        #[holder(use_place_holder)]
        pub open_shell_element: OpenShellAny,
        pub orientation: bool,
    }
    impl Into<OpenShellAny> for OrientedOpenShell {
        fn into(self) -> OpenShellAny {
            OpenShellAny::OrientedOpenShell(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = closed_shell)]
    #[holder(generate_deserialize)]
    pub struct ClosedShell {
        #[holder(use_place_holder)]
        pub connected_face_set: ConnectedFaceSet,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ClosedShellAny {
        #[holder(use_place_holder)]
        # [holder (field = oriented_closed_shell)]
        OrientedClosedShell(Box<OrientedClosedShell>),
    }
    impl Into<ConnectedFaceSetAny> for ClosedShellAny {
        fn into(self) -> ConnectedFaceSetAny {
            ConnectedFaceSetAny::ClosedShellAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = oriented_closed_shell)]
    #[holder(generate_deserialize)]
    pub struct OrientedClosedShell {
        #[holder(use_place_holder)]
        pub closed_shell: ClosedShell,
        #[holder(use_place_holder)]
        pub closed_shell_element: ClosedShellAny,
        pub orientation: bool,
    }
    impl Into<ClosedShellAny> for OrientedClosedShell {
        fn into(self) -> ClosedShellAny {
            ClosedShellAny::OrientedClosedShell(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = connected_edge_set)]
    #[holder(generate_deserialize)]
    pub struct ConnectedEdgeSet {
        #[holder(use_place_holder)]
        pub topological_representation_item: TopologicalRepresentationItem,
        #[holder(use_place_holder)]
        pub ces_edges: Vec<EdgeAny>,
    }
    impl Into<TopologicalRepresentationItemAny> for ConnectedEdgeSet {
        fn into(self) -> TopologicalRepresentationItemAny {
            TopologicalRepresentationItemAny::ConnectedEdgeSet(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_model)]
    #[holder(generate_deserialize)]
    pub struct SolidModel {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SolidModelAny {
        #[holder(use_place_holder)]
        # [holder (field = manifold_solid_brep)]
        ManifoldSolidBrepAny(Box<ManifoldSolidBrepAny>),
        #[holder(use_place_holder)]
        # [holder (field = csg_solid)]
        CsgSolid(Box<CsgSolid>),
        #[holder(use_place_holder)]
        # [holder (field = swept_face_solid)]
        SweptFaceSolidAny(Box<SweptFaceSolidAny>),
        #[holder(use_place_holder)]
        # [holder (field = swept_area_solid)]
        SweptAreaSolidAny(Box<SweptAreaSolidAny>),
        #[holder(use_place_holder)]
        # [holder (field = solid_replica)]
        SolidReplica(Box<SolidReplica>),
    }
    impl Into<GeometricRepresentationItemAny> for SolidModelAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::SolidModelAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = manifold_solid_brep)]
    #[holder(generate_deserialize)]
    pub struct ManifoldSolidBrep {
        #[holder(use_place_holder)]
        pub solid_model: SolidModel,
        #[holder(use_place_holder)]
        pub outer: ClosedShellAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum ManifoldSolidBrepAny {
        #[holder(use_place_holder)]
        # [holder (field = brep_with_voids)]
        BrepWithVoids(Box<BrepWithVoids>),
        #[holder(use_place_holder)]
        # [holder (field = faceted_brep)]
        FacetedBrep(Box<FacetedBrep>),
    }
    impl Into<SolidModelAny> for ManifoldSolidBrepAny {
        fn into(self) -> SolidModelAny {
            SolidModelAny::ManifoldSolidBrepAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = brep_with_voids)]
    #[holder(generate_deserialize)]
    pub struct BrepWithVoids {
        #[holder(use_place_holder)]
        pub manifold_solid_brep: ManifoldSolidBrep,
        #[holder(use_place_holder)]
        pub voids: Vec<OrientedClosedShell>,
    }
    impl Into<ManifoldSolidBrepAny> for BrepWithVoids {
        fn into(self) -> ManifoldSolidBrepAny {
            ManifoldSolidBrepAny::BrepWithVoids(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = faceted_brep)]
    #[holder(generate_deserialize)]
    pub struct FacetedBrep {
        #[holder(use_place_holder)]
        pub manifold_solid_brep: ManifoldSolidBrep,
    }
    impl Into<ManifoldSolidBrepAny> for FacetedBrep {
        fn into(self) -> ManifoldSolidBrepAny {
            ManifoldSolidBrepAny::FacetedBrep(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = csg_solid)]
    #[holder(generate_deserialize)]
    pub struct CsgSolid {
        #[holder(use_place_holder)]
        pub solid_model: SolidModel,
        #[holder(use_place_holder)]
        pub tree_root_expression: CsgSelect,
    }
    impl Into<SolidModelAny> for CsgSolid {
        fn into(self) -> SolidModelAny {
            SolidModelAny::CsgSolid(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = boolean_result)]
    #[holder(generate_deserialize)]
    pub struct BooleanResult {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        pub operator: BooleanOperator,
        #[holder(use_place_holder)]
        pub first_operand: BooleanOperand,
        #[holder(use_place_holder)]
        pub second_operand: BooleanOperand,
    }
    impl Into<GeometricRepresentationItemAny> for BooleanResult {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::BooleanResult(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = sphere)]
    #[holder(generate_deserialize)]
    pub struct Sphere {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        pub radius: PositiveLengthMeasure,
        #[holder(use_place_holder)]
        pub centre: PointAny,
    }
    impl Into<GeometricRepresentationItemAny> for Sphere {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Sphere(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = right_circular_cone)]
    #[holder(generate_deserialize)]
    pub struct RightCircularCone {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub position: Axis1Placement,
        pub height: PositiveLengthMeasure,
        pub radius: LengthMeasure,
        pub semi_angle: PlaneAngleMeasure,
    }
    impl Into<GeometricRepresentationItemAny> for RightCircularCone {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::RightCircularCone(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = right_circular_cylinder)]
    #[holder(generate_deserialize)]
    pub struct RightCircularCylinder {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub position: Axis1Placement,
        pub height: PositiveLengthMeasure,
        pub radius: PositiveLengthMeasure,
    }
    impl Into<GeometricRepresentationItemAny> for RightCircularCylinder {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::RightCircularCylinder(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = torus)]
    #[holder(generate_deserialize)]
    pub struct Torus {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub position: Axis1Placement,
        pub major_radius: PositiveLengthMeasure,
        pub minor_radius: PositiveLengthMeasure,
    }
    impl Into<GeometricRepresentationItemAny> for Torus {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Torus(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = block)]
    #[holder(generate_deserialize)]
    pub struct Block {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
        pub x: PositiveLengthMeasure,
        pub y: PositiveLengthMeasure,
        pub z: PositiveLengthMeasure,
    }
    impl Into<GeometricRepresentationItemAny> for Block {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::Block(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = right_angular_wedge)]
    #[holder(generate_deserialize)]
    pub struct RightAngularWedge {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub position: Axis2Placement3D,
        pub x: PositiveLengthMeasure,
        pub y: PositiveLengthMeasure,
        pub z: PositiveLengthMeasure,
        pub ltx: LengthMeasure,
    }
    impl Into<GeometricRepresentationItemAny> for RightAngularWedge {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::RightAngularWedge(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = swept_face_solid)]
    #[holder(generate_deserialize)]
    pub struct SweptFaceSolid {
        #[holder(use_place_holder)]
        pub solid_model: SolidModel,
        #[holder(use_place_holder)]
        pub swept_face: FaceSurface,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SweptFaceSolidAny {
        #[holder(use_place_holder)]
        # [holder (field = extruded_face_solid)]
        ExtrudedFaceSolid(Box<ExtrudedFaceSolid>),
        #[holder(use_place_holder)]
        # [holder (field = revolved_face_solid)]
        RevolvedFaceSolid(Box<RevolvedFaceSolid>),
    }
    impl Into<SolidModelAny> for SweptFaceSolidAny {
        fn into(self) -> SolidModelAny {
            SolidModelAny::SweptFaceSolidAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = extruded_face_solid)]
    #[holder(generate_deserialize)]
    pub struct ExtrudedFaceSolid {
        #[holder(use_place_holder)]
        pub swept_face_solid: SweptFaceSolid,
        #[holder(use_place_holder)]
        pub extruded_direction: Direction,
        pub depth: PositiveLengthMeasure,
    }
    impl Into<SweptFaceSolidAny> for ExtrudedFaceSolid {
        fn into(self) -> SweptFaceSolidAny {
            SweptFaceSolidAny::ExtrudedFaceSolid(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = revolved_face_solid)]
    #[holder(generate_deserialize)]
    pub struct RevolvedFaceSolid {
        #[holder(use_place_holder)]
        pub swept_face_solid: SweptFaceSolid,
        #[holder(use_place_holder)]
        pub axis: Axis1Placement,
        pub angle: PlaneAngleMeasure,
    }
    impl Into<SweptFaceSolidAny> for RevolvedFaceSolid {
        fn into(self) -> SweptFaceSolidAny {
            SweptFaceSolidAny::RevolvedFaceSolid(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = swept_area_solid)]
    #[holder(generate_deserialize)]
    pub struct SweptAreaSolid {
        #[holder(use_place_holder)]
        pub solid_model: SolidModel,
        #[holder(use_place_holder)]
        pub swept_area: CurveBoundedSurface,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum SweptAreaSolidAny {
        #[holder(use_place_holder)]
        # [holder (field = extruded_area_solid)]
        ExtrudedAreaSolid(Box<ExtrudedAreaSolid>),
        #[holder(use_place_holder)]
        # [holder (field = revolved_area_solid)]
        RevolvedAreaSolid(Box<RevolvedAreaSolid>),
    }
    impl Into<SolidModelAny> for SweptAreaSolidAny {
        fn into(self) -> SolidModelAny {
            SolidModelAny::SweptAreaSolidAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = extruded_area_solid)]
    #[holder(generate_deserialize)]
    pub struct ExtrudedAreaSolid {
        #[holder(use_place_holder)]
        pub swept_area_solid: SweptAreaSolid,
        #[holder(use_place_holder)]
        pub extruded_direction: Direction,
        pub depth: PositiveLengthMeasure,
    }
    impl Into<SweptAreaSolidAny> for ExtrudedAreaSolid {
        fn into(self) -> SweptAreaSolidAny {
            SweptAreaSolidAny::ExtrudedAreaSolid(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = revolved_area_solid)]
    #[holder(generate_deserialize)]
    pub struct RevolvedAreaSolid {
        #[holder(use_place_holder)]
        pub swept_area_solid: SweptAreaSolid,
        #[holder(use_place_holder)]
        pub axis: Axis1Placement,
        pub angle: PlaneAngleMeasure,
    }
    impl Into<SweptAreaSolidAny> for RevolvedAreaSolid {
        fn into(self) -> SweptAreaSolidAny {
            SweptAreaSolidAny::RevolvedAreaSolid(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = half_space_solid)]
    #[holder(generate_deserialize)]
    pub struct HalfSpaceSolid {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub base_surface: SurfaceAny,
        pub agreement_flag: bool,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum HalfSpaceSolidAny {
        #[holder(use_place_holder)]
        # [holder (field = boxed_half_space)]
        BoxedHalfSpace(Box<BoxedHalfSpace>),
    }
    impl Into<GeometricRepresentationItemAny> for HalfSpaceSolidAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::HalfSpaceSolidAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = boxed_half_space)]
    #[holder(generate_deserialize)]
    pub struct BoxedHalfSpace {
        #[holder(use_place_holder)]
        pub half_space_solid: HalfSpaceSolid,
        #[holder(use_place_holder)]
        pub enclosure: BoxDomain,
    }
    impl Into<HalfSpaceSolidAny> for BoxedHalfSpace {
        fn into(self) -> HalfSpaceSolidAny {
            HalfSpaceSolidAny::BoxedHalfSpace(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = box_domain)]
    #[holder(generate_deserialize)]
    pub struct BoxDomain {
        #[holder(use_place_holder)]
        pub founded_item: FoundedItem,
        #[holder(use_place_holder)]
        pub corner: CartesianPointAny,
        pub xlength: PositiveLengthMeasure,
        pub ylength: PositiveLengthMeasure,
        pub zlength: PositiveLengthMeasure,
    }
    impl Into<FoundedItemAny> for BoxDomain {
        fn into(self) -> FoundedItemAny {
            FoundedItemAny::BoxDomain(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = solid_replica)]
    #[holder(generate_deserialize)]
    pub struct SolidReplica {
        #[holder(use_place_holder)]
        pub solid_model: SolidModel,
        #[holder(use_place_holder)]
        pub parent_solid: SolidModelAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperator3D,
    }
    impl Into<SolidModelAny> for SolidReplica {
        fn into(self) -> SolidModelAny {
            SolidModelAny::SolidReplica(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_surface_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedSurfaceModel {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub sbsm_boundary: Vec<Shell>,
    }
    impl Into<GeometricRepresentationItemAny> for ShellBasedSurfaceModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::ShellBasedSurfaceModel(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = face_based_surface_model)]
    #[holder(generate_deserialize)]
    pub struct FaceBasedSurfaceModel {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub fbsm_faces: Vec<ConnectedFaceSetAny>,
    }
    impl Into<GeometricRepresentationItemAny> for FaceBasedSurfaceModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::FaceBasedSurfaceModel(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = shell_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct ShellBasedWireframeModel {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub sbwm_boundary: Vec<Shell>,
    }
    impl Into<GeometricRepresentationItemAny> for ShellBasedWireframeModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::ShellBasedWireframeModel(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = edge_based_wireframe_model)]
    #[holder(generate_deserialize)]
    pub struct EdgeBasedWireframeModel {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub ebwm_boundary: Vec<ConnectedEdgeSet>,
    }
    impl Into<GeometricRepresentationItemAny> for EdgeBasedWireframeModel {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::EdgeBasedWireframeModel(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricSet {
        #[holder(use_place_holder)]
        pub geometric_representation_item: GeometricRepresentationItem,
        #[holder(use_place_holder)]
        pub elements: Vec<GeometricSetSelect>,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum GeometricSetAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_curve_set)]
        GeometricCurveSet(Box<GeometricCurveSet>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_set_replica)]
        GeometricSetReplica(Box<GeometricSetReplica>),
    }
    impl Into<GeometricRepresentationItemAny> for GeometricSetAny {
        fn into(self) -> GeometricRepresentationItemAny {
            GeometricRepresentationItemAny::GeometricSetAny(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_curve_set)]
    #[holder(generate_deserialize)]
    pub struct GeometricCurveSet {
        #[holder(use_place_holder)]
        pub geometric_set: GeometricSet,
    }
    impl Into<GeometricSetAny> for GeometricCurveSet {
        fn into(self) -> GeometricSetAny {
            GeometricSetAny::GeometricCurveSet(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = geometric_set_replica)]
    #[holder(generate_deserialize)]
    pub struct GeometricSetReplica {
        #[holder(use_place_holder)]
        pub geometric_set: GeometricSet,
        #[holder(use_place_holder)]
        pub parent_set: GeometricSetAny,
        #[holder(use_place_holder)]
        pub transformation: CartesianTransformationOperatorAny,
    }
    impl Into<GeometricSetAny> for GeometricSetReplica {
        fn into(self) -> GeometricSetAny {
            GeometricSetAny::GeometricSetReplica(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = uncertainty_measure_with_unit)]
    #[holder(generate_deserialize)]
    pub struct UncertaintyMeasureWithUnit {
        #[holder(use_place_holder)]
        pub measure_with_unit: MeasureWithUnit,
        pub name: Label,
        pub description: Text,
    }
    impl Into<MeasureWithUnitAny> for UncertaintyMeasureWithUnit {
        fn into(self) -> MeasureWithUnitAny {
            MeasureWithUnitAny::UncertaintyMeasureWithUnit(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_context)]
    #[holder(generate_deserialize)]
    pub struct RepresentationContext {
        pub context_identifier: Identifier,
        pub context_type: Text,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationContextAny {
        #[holder(use_place_holder)]
        # [holder (field = global_unit_assigned_context)]
        GlobalUnitAssignedContext(Box<GlobalUnitAssignedContext>),
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_context)]
        GeometricRepresentationContext(Box<GeometricRepresentationContext>),
        #[holder(use_place_holder)]
        # [holder (field = global_uncertainty_assigned_context)]
        GlobalUncertaintyAssignedContext(Box<GlobalUncertaintyAssignedContext>),
        #[holder(use_place_holder)]
        # [holder (field = parametric_representation_context)]
        ParametricRepresentationContext(Box<ParametricRepresentationContext>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = global_uncertainty_assigned_context)]
    #[holder(generate_deserialize)]
    pub struct GlobalUncertaintyAssignedContext {
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
        #[holder(use_place_holder)]
        pub uncertainty: Vec<UncertaintyMeasureWithUnit>,
    }
    impl Into<RepresentationContextAny> for GlobalUncertaintyAssignedContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::GlobalUncertaintyAssignedContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_item)]
    #[holder(generate_deserialize)]
    pub struct RepresentationItem {
        pub name: Label,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationItemAny {
        #[holder(use_place_holder)]
        # [holder (field = geometric_representation_item)]
        GeometricRepresentationItemAny(Box<GeometricRepresentationItemAny>),
        #[holder(use_place_holder)]
        # [holder (field = topological_representation_item)]
        TopologicalRepresentationItemAny(Box<TopologicalRepresentationItemAny>),
        #[holder(use_place_holder)]
        # [holder (field = mapped_item)]
        MappedItem(Box<MappedItem>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation)]
    #[holder(generate_deserialize)]
    pub struct Representation {
        pub name: Label,
        #[holder(use_place_holder)]
        pub items: Vec<RepresentationItemAny>,
        #[holder(use_place_holder)]
        pub context_of_items: RepresentationContextAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationAny {
        #[holder(use_place_holder)]
        # [holder (field = shape_representation)]
        ShapeRepresentation(Box<ShapeRepresentation>),
        #[holder(use_place_holder)]
        # [holder (field = definitional_representation)]
        DefinitionalRepresentation(Box<DefinitionalRepresentation>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationship {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub rep_1: RepresentationAny,
        #[holder(use_place_holder)]
        pub rep_2: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum RepresentationRelationshipAny {
        #[holder(use_place_holder)]
        # [holder (field = shape_representation_relationship)]
        ShapeRepresentationRelationship(Box<ShapeRepresentationRelationship>),
        #[holder(use_place_holder)]
        # [holder (field = representation_relationship_with_transformation)]
        RepresentationRelationshipWithTransformation(
            Box<RepresentationRelationshipWithTransformation>,
        ),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = item_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct ItemDefinedTransformation {
        pub name: Label,
        pub description: Text,
        #[holder(use_place_holder)]
        pub transform_item_1: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub transform_item_2: RepresentationItemAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = functionally_defined_transformation)]
    #[holder(generate_deserialize)]
    pub struct FunctionallyDefinedTransformation {
        pub name: Label,
        pub description: Text,
    }
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FunctionallyDefinedTransformationAny {
        #[holder(use_place_holder)]
        # [holder (field = cartesian_transformation_operator)]
        CartesianTransformationOperatorAny(Box<CartesianTransformationOperatorAny>),
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_relationship_with_transformation)]
    #[holder(generate_deserialize)]
    pub struct RepresentationRelationshipWithTransformation {
        #[holder(use_place_holder)]
        pub representation_relationship: RepresentationRelationship,
        #[holder(use_place_holder)]
        pub transformation_operator: Transformation,
    }
    impl Into<RepresentationRelationshipAny> for RepresentationRelationshipWithTransformation {
        fn into(self) -> RepresentationRelationshipAny {
            RepresentationRelationshipAny::RepresentationRelationshipWithTransformation(Box::new(
                self,
            ))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = representation_map)]
    #[holder(generate_deserialize)]
    pub struct RepresentationMap {
        #[holder(use_place_holder)]
        pub mapping_origin: RepresentationItemAny,
        #[holder(use_place_holder)]
        pub mapped_representation: RepresentationAny,
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = mapped_item)]
    #[holder(generate_deserialize)]
    pub struct MappedItem {
        #[holder(use_place_holder)]
        pub representation_item: RepresentationItem,
        #[holder(use_place_holder)]
        pub mapping_source: RepresentationMap,
        #[holder(use_place_holder)]
        pub mapping_target: RepresentationItemAny,
    }
    impl Into<RepresentationItemAny> for MappedItem {
        fn into(self) -> RepresentationItemAny {
            RepresentationItemAny::MappedItem(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = definitional_representation)]
    #[holder(generate_deserialize)]
    pub struct DefinitionalRepresentation {
        #[holder(use_place_holder)]
        pub representation: Representation,
    }
    impl Into<RepresentationAny> for DefinitionalRepresentation {
        fn into(self) -> RepresentationAny {
            RepresentationAny::DefinitionalRepresentation(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = parametric_representation_context)]
    #[holder(generate_deserialize)]
    pub struct ParametricRepresentationContext {
        #[holder(use_place_holder)]
        pub representation_context: RepresentationContext,
    }
    impl Into<RepresentationContextAny> for ParametricRepresentationContext {
        fn into(self) -> RepresentationContextAny {
            RepresentationContextAny::ParametricRepresentationContext(Box::new(self))
        }
    }
    #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
    # [holder (table = Tables)]
    # [holder (field = founded_item)]
    #[holder(generate_deserialize)]
    pub struct FoundedItem {}
    #[derive(Debug, Clone, PartialEq, Holder)]
    # [holder (table = Tables)]
    #[holder(generate_deserialize)]
    pub enum FoundedItemAny {
        #[holder(use_place_holder)]
        # [holder (field = composite_curve_segment)]
        CompositeCurveSegmentAny(Box<CompositeCurveSegmentAny>),
        #[holder(use_place_holder)]
        # [holder (field = surface_patch)]
        SurfacePatch(Box<SurfacePatch>),
        #[holder(use_place_holder)]
        # [holder (field = box_domain)]
        BoxDomain(Box<BoxDomain>),
    }
}
